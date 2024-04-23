use std::ops::{AddAssign, Div, Neg};

use crate::api::endpoints::mv::EllipticCurveBean;
use anyhow::{Context, Result};
use atomic_counter::RelaxedCounter;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::{One, Signed, Zero};
use log::warn;
use num::Integer;

use crate::math_core::complex_number::{complex_euclidean_algorithm, ComplexNumber};
use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::math_core::traits::divisible::Divisible;
use crate::math_core::traits::increment::Increment;

///
/// Repräsentiert eine elliptische Kurve mit einer zyklischen Untergruppe, in der das
/// Problem des diskreten Logarithmus praktisch nicht lösbar ist.
///
/// Die elliptische Kurve ist über die Gleichung y^2 = x^3 + a · x definiert.
/// Dabei ist a = -n^2, wobei n eine Ganzzahl ungleich 0 ist.
/// Um die Kurve über einem endlichen Körper zu definieren, wird auch der Modulus p benötigt.
///
#[derive(Clone, PartialEq, Debug)]
pub struct SecureFiniteFieldEllipticCurve {
    /// Der Koeffizient a der elliptischen Kurve
    pub a: i64,
    /// Der Modulus p der elliptischen Kurve, um sie über einem endlichen Körper zu definieren
    pub prime: BigInt,
    /// Die Ordnung der zyklischen Untergruppe / des Generators, in welcher das Problem des
    /// diskreten Logarithmus praktisch nicht lösbar ist
    pub order_of_subgroup: BigInt,
    /// Der Generator der zyklischen Untergruppe
    pub generator: FiniteFieldEllipticCurvePoint,
}

impl From<EllipticCurveBean> for SecureFiniteFieldEllipticCurve {
    /// Mapped die Bean in das Domain-Modell
    fn from(curve: EllipticCurveBean) -> Self {
        SecureFiniteFieldEllipticCurve {
            a: curve.a,
            prime: curve.prime.parse().unwrap(),
            order_of_subgroup: curve.order_of_subgroup.parse().unwrap(),
            generator: FiniteFieldEllipticCurvePoint::from(curve.generator),
        }
    }
}

impl SecureFiniteFieldEllipticCurve {
    /// Erstellt eine neue elliptische Kurve der Form y^2 = x^3 + (-n^2)*x (mod p) unter
    /// Angabe von n und der bitbreite des Modulus p.
    /// Die Kurve wird dabei kryptografisch sicher sein und dafür eine Reihe von Bedingungen
    /// erfüllen:
    /// - Es muss gelten n > 0,
    /// - Es muss für den Modulus p gelten, dass
    /// -- p mod 8 = 5 und
    /// -- p nicht 2*n teilt (2n mod p != 0)
    /// - Eine zyklische Untergruppe der Ordnung q muss existieren, wobei für q gilt:
    /// -- q = N / 8, wobei N = |E(Z_p)| (Ordnung der Kurve) und
    /// -- q muss eine Primzahl sein
    pub fn new(n: i64, modul_width: u32, miller_rabin_iterations: u32) -> Result<Self> {
        if n.is_zero() {
            panic!("Der Koeffizient a darf nicht 0 sein!"); // TODO Error Handling
        }
        if modul_width < 4u32 {
            panic!("Der Modulus p muss mindestens 4 Bit breit sein!"); // TODO Error Handling
        }
        let a = n.pow(2).neg();

        // Wird für einen späteren Vergleich benötigt
        let double_n = BigInt::from(n).double();

        let prng = PseudoRandomNumberGenerator::new_seeded();
        let counter = RelaxedCounter::new(1);
        let mut prime: BigInt;
        loop {
            prime = prng.generate_prime(modul_width, miller_rabin_iterations, &counter);
            // Die Primzahl muss mod 8 kongruent 5 genügen und darf 2n nicht teilen
            if prime.rem_euclid(&8.into()) == 5.into() && !double_n.is_multiple_of(&prime) {
                break;
            }
        }

        let counter = RelaxedCounter::new(1);

        // Manchmal wird ein Generator bestimmt, der nicht auf der Kurve liegt. In dem Fall soll
        // die Berechnung wiederholt werden, bis ein gültiger Generator gefunden wurde.
        loop {
            let (prime, order_of_subgroup) =
                Self::calculate_p_and_q(&prime, n, miller_rabin_iterations);

            let curve = Self {
                a,
                prime: prime.clone(),
                order_of_subgroup: order_of_subgroup.clone(),
                generator: Default::default(),
            };
            // Hier kann die Kurve mit dem Defaultwert für den Generator mitgegeben werden, weil dieser
            // für das Multiplizieren in der Bestimmung des Generators nicht notwendig ist.
            // Anschließend wird die Kurve mit dem Generator neu erstellt.
            let generator = Self::calculate_signature_generator(
                &prime,
                a,
                &order_of_subgroup,
                &curve,
                &counter,
            )
            .context("Error while calculating signature generator")?;

            let curve = Self {
                a,
                prime,
                order_of_subgroup,
                generator,
            };

            if curve.has_point(&curve.generator) {
                return Ok(curve);
            }
            warn!(
                "Berechneter Generator ist kein Punkt der Kurve! \
            Es wird ein neuer Generator berechnet."
            );
        }
    }

    pub fn calculate_p_and_q(
        prime: &BigInt,
        n: i64,
        miller_rabin_iterations: u32,
    ) -> (BigInt, BigInt) {
        let double_n = BigInt::from(n).double();
        let mut prime = prime.clone();
        let mut q: BigInt;
        let service = NumberTheoryService::new(Fast); // TODO übergeben lassen
        let prng = PseudoRandomNumberGenerator::new_seeded(); // TODO übergeben lassen

        // Die Schleife, die läuft, bis 'q = N / 8' eine Primzahl ergibt.
        loop {
            // Die Schleife, die eine passende Primzahl bestimmt.
            loop {
                // Wenn die Primzahl folgende Bedingungen erfüllt, so genügt sie dem Verfahren:
                // 1. Sie ist eine Primzahl
                // 2. Sie ist ein quadratischer Rest zu p, also n^((p-1)/2) = 1 (mod p)
                // 3. Sie ist kein Vielfaches von 2n
                if service.is_probably_prime(&prime, miller_rabin_iterations, &prng)
                    && service
                        .fast_exponentiation(&n.into(), &prime.decrement().half(), &prime)
                        .is_one()
                    && !double_n.is_multiple_of(&prime)
                {
                    break;
                }
                // Treffen diese Bedingungen nicht zu, wird kongruenzerhaltend eine neue getestet.
                prime.add_assign(BigInt::from(8));
            }

            let big_n = Self::calculate_big_n(&prime, n);

            q = big_n.div(8);
            // Ist q = N / 8 eine Primzahl, so wird die Schleife verlassen und das q ist gültig.
            if service.is_probably_prime(&q, miller_rabin_iterations, &prng) {
                return (prime, q);
            }
            // Ist q keine Primzahl, wird prime um 8 erhöht und ein neuer Versuch gestartet.
            // Es wird (wie oben auch) um 8 erhöht, da p = 5 (mod 8) gelten muss.
            prime.add_assign(BigInt::from(8));
        }
    }

    fn calculate_big_n(prime: &BigInt, n: i64) -> BigInt {
        let first_complex_number = ComplexNumber::new(prime.clone(), BigInt::zero());
        let second_complex_number =
            ComplexNumber::new(Self::calculate_w(&prime, 2.into()), BigInt::one());
        let gg_t: ComplexNumber =
            complex_euclidean_algorithm(first_complex_number, second_complex_number);

        // Der Realteil von alpha ist immer der ungerade Anteil des ggT von p und W(p, 2)
        // dadurch, dass das obige Verfahren immer einen geraden und ungeraden Anteil liefert,
        // lässt sich alpha problemlos bestimmen. D.W. muss alpha die Absolutwerte enthalten.
        let alpha: ComplexNumber;
        if gg_t.real.is_even() {
            alpha = ComplexNumber::new(gg_t.imaginary.clone().abs(), gg_t.real.clone().abs());
        } else {
            alpha = ComplexNumber::new(gg_t.real.clone().abs(), gg_t.imaginary.clone().abs());
        }

        prime.increment() - Self::calculate_real_part(alpha, &prime, n).double()
    }

    pub fn calculate_w(prime: &BigInt, z: BigInt) -> BigInt {
        let mut z: BigInt = z.clone();
        // w(p, z) = z ^ ((p - 1) / 4) (mod p)
        // gilt anschließend w(p, z)^2 + 1 = 0 (mod p), ist der Wert gültig.
        // Andernfalls wiederholen mit z = z + 2
        let service = NumberTheoryService::new(Fast); // TODO übergeben lassen
        let mut w: BigInt;
        loop {
            w = service.fast_exponentiation(&z, &(prime.decrement().div(4)), prime);
            if (w.pow(2) + BigInt::one()).rem_euclid(prime).is_zero() {
                // TODO gegen Service.fastExponentiation austauschen
                break;
            }
            z.add_assign(BigInt::from(2));
        }
        w
    }

    pub fn calculate_real_part(alpha: ComplexNumber, prime: &BigInt,    n: i64) -> BigInt {
        let mut count = 4;
        let mut alpha = alpha.clone();
        // Schleife, die alle möglichen Konjugationen von alpha durchgeht
        loop {
            let complex_legendre_symbol =
                ComplexNumber::new(Self::calculate_legendre_symbol(&n.into(), prime), 0.into());
            let two_two = ComplexNumber::new(2.into(), 2.into());
            // Produkt aus der Differenz von alpha und dessen Legendre-Symbol und dem konjugierten Wert von 2 + 2i
            let product = (&alpha - &complex_legendre_symbol) * two_two.conjugate();

            // Rückgabe des validen Realteils von alpha
            if product.real.rem_euclid(&8.into()).is_zero() {
                return alpha.real;
            }
            // Der "Fehlschlag" wird gezählt
            count -= 1;
            if count == 0 {
                panic!("Es konnte kein gültiger Realteil für alpha gefunden werden!");
            }
            // Liegt alpha im ersten oder dritten Quadranten, wird es in den zweiten oder vierten
            // Quadranten verschoben
            if alpha.is_in_first_quadrant() || alpha.is_in_third_quadrant() {
                alpha = alpha.negate().conjugate();
            } else {
                // Liegt alpha im zweiten oder vierten Quadranten, wird es in den ersten oder dritten
                // Quadranten verschoben
                alpha = alpha.conjugate();
            }
        }
    }

    //TODO Doku: Nach Satz 1.15 und Definition 1.27 (Kryptographie 2)
    //TODO Auch aufnehmen, dass b eine Primzahl > 3 sein muss
    pub fn calculate_legendre_symbol(a: &BigInt, prime: &BigInt) -> BigInt {
        let service = NumberTheoryService::new(Fast); // TODO übergeben lassen

        // TODO: Fall von b teilt a = 0 zurückgeben
        // Danach kann das Kriterium auf Rückgabe von fastExp(a, prime/2 -1, prime) reduziert werden, weil
        // nur noch 1 und -1 als Ergebnis herauskommen können.

        let negative_one = BigInt::from(-1);
        if a == &prime.decrement() {
            // Satz 1.18
            let exponent: BigInt = prime.decrement().div(2);
            return if exponent.is_even() {
                BigInt::one()
            } else {
                negative_one
            };
        }

        if a == &BigInt::from(2) {
            // Satz 1.19
            let exponent: BigInt = prime.pow(2).decrement().div(8);
            return if exponent.is_even() {
                BigInt::one()
            } else {
                negative_one
            };
        }

        // legendre_symbol = a ^ ((b - 1) / 2) (mod b)
        let legendre_symbol = service.fast_exponentiation(&a, &prime.decrement().half(), &prime);
        if legendre_symbol.is_one() {
            BigInt::one()
        } else {
            negative_one
        }
    }

    pub fn calculate_signature_generator(
        prime: &BigInt,
        a: i64,
        q: &BigInt,
        curve: &SecureFiniteFieldEllipticCurve,
        counter: &RelaxedCounter,
    ) -> Result<FiniteFieldEllipticCurvePoint> {
        let mut generator: FiniteFieldEllipticCurvePoint;
        let service = NumberTheoryService::new(Fast); // TODO übergeben lassen

        // Schleife, die läuft, bis ein Generator gefunden wurde, der nicht den Punkt im Unendlichen
        // darstellt oder dessen Ordnung nicht N/8 ist.
        loop {
            let prng = PseudoRandomNumberGenerator::new_seeded(); // TODO übergeben lassen
            let (mut x, mut r);
            // Schleife, die bis zum Fund eines validen quadratischen Rests läuft
            loop {
                x = prng.take(&BigInt::one(), &prime.decrement(), &counter);
                r = service.fast_exponentiation(&x, &BigInt::from(3), prime) + a * &x;
                // Kriterium für den quadratischen Rest
                if service
                    .fast_exponentiation(&r, &prime.decrement().half(), &prime)
                    .is_one()
                {
                    break;
                }
            }

            // Bedingung, anhand derer bestimmt wird, welche der beiden Formeln nach Satz 4.1 zu
            // verwenden ist
            let condition = service.fast_exponentiation(&r, &prime.decrement().div(4), &prime);
            let y: BigInt;
            let exponent: BigInt = (prime + BigInt::from(3)).div(8);
            if condition.is_one() {
                y = service.fast_exponentiation(&r, &exponent, &prime);
            } else {
                y = service
                    .fast_exponentiation(&(BigInt::from(4) * r), &exponent, &prime)
                    .half();
            }
            // Den Generator mit den berechnen Koordinaten erstellen und prüfen.
            generator = FiniteFieldEllipticCurvePoint::new(x, y);

            // Falls der generierte Punkt nicht auf der Kurve liegt, wird ein neuer Punkt generiert.
            if !curve.has_point(&generator) {
                continue;
            }

            // Der Generator selber darf nicht im Unendlichen liegen und auch die Ordnung
            // des Punktes muss gleich q sein, also muss Generator*q im Unendlichen liegen.
            if !generator.is_infinite && generator.multiply(q, curve)?.is_infinite {
                break;
            }
        }
        Ok(generator)
    }

    ///
    /// Überprüft, ob ein Punkt auf der elliptischen Kurve liegt.
    ///
    pub fn has_point(&self, point: &FiniteFieldEllipticCurvePoint) -> bool {
        let x_squared = &point.x.pow(2);
        let x_cubed = &point.x * x_squared;
        let y_squared = point.y.pow(2);

        // y^2 = x^3 + ax (mod p) ist äquivalent zu (x^3 + ax - y^2) % p == 0
        let remainder = (x_cubed + &self.a * &point.x - y_squared).rem_euclid(&self.prime);
        remainder == BigInt::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_big_n() {
        let mut prime = BigInt::from(17);
        let n = 2;
        let big_n = SecureFiniteFieldEllipticCurve::calculate_big_n(&mut prime, n);
        assert_eq!(big_n, BigInt::from(16));

        let mut prime = BigInt::from(13);
        let n = 1;
        let big_n = SecureFiniteFieldEllipticCurve::calculate_big_n(&mut prime, n);
        assert_eq!(big_n, BigInt::from(8));

        let mut prime = BigInt::from(17);
        let n = 1;
        let big_n = SecureFiniteFieldEllipticCurve::calculate_big_n(&mut prime, n);
        assert_eq!(big_n, BigInt::from(16));

        let mut prime = BigInt::from(13);
        let n = 3;
        let big_n = SecureFiniteFieldEllipticCurve::calculate_big_n(&mut prime, n);
        assert_eq!(big_n, BigInt::from(8));

        let mut prime = BigInt::from(13);
        let n = 2;
        let big_n = SecureFiniteFieldEllipticCurve::calculate_big_n(&mut prime, n);
        assert_eq!(big_n, BigInt::from(20));

        let mut prime = BigInt::from(509);
        let n = 2;
        let big_n = SecureFiniteFieldEllipticCurve::calculate_big_n(&mut prime, n);
        assert_eq!(big_n, BigInt::from(500));
    }

    #[test]
    fn test_has_point_not() {
        let curve = SecureFiniteFieldEllipticCurve::new(7, 17, 20).unwrap();
        let point = FiniteFieldEllipticCurvePoint::new(5.into(), 7.into());
        // (5, 7) liegt nicht auf y^2 = x^3 + 7 (mod 17)
        assert!(!curve.has_point(&point));

        let point = FiniteFieldEllipticCurvePoint::new(4.into(), 6.into());
        // (4, 6) liegt nicht auf y^2 = x^3 + 7 (mod 17). Genaugenommen tut es keiner mit x=4.
        assert!(!curve.has_point(&point));
    }

    #[test]
    fn test_has_point() {
        let curve = SecureFiniteFieldEllipticCurve::new(5, 16, 40).unwrap();
        let point = curve.generator.multiply(&3.into(), &curve).unwrap();
        // (5, 8) liegt auf y^2 = x^3 + 7 (mod 17)
        assert!(curve.has_point(&point));

        let point = FiniteFieldEllipticCurvePoint::new(0.into(), 0.into());
        // (5, 8) liegt auf y^2 = x^3 + 7 (mod 17)
        assert!(curve.has_point(&point));
    }
}
