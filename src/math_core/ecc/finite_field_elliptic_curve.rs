use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::Zero;
use std::ops::Neg;

use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;

///
/// Repräsentiert eine elliptische Kurve.
///
/// Die Koeffizienten a und b der elliptischen Kurve sind die Koeffizienten der Gleichung
/// y^2 = x^3 + ax + b, die die elliptische Kurve definiert.
/// Um die Kurve über einem endlichen Körper zu definieren, wird auch der Modulus p benötigt.
///
#[derive(Clone, PartialEq, Debug)]
    // Die Koeffizienten a und b der elliptischen Kurve
pub struct SecureFiniteFieldEllipticCurve {
    pub a: i32,
    pub b: i32,
    // Der Modulus p der elliptischen Kurve, um sie über einem endlichen Körper zu definieren
    pub prime: BigInt,
}

impl SecureFiniteFieldEllipticCurve {
    /// Erstellt eine neue elliptische Kurve der Form y^2 = x^3 + (-n^2)*x (mod p) unter
    /// Angabe von n und der bitbreite des Modulus p.
    /// Die Kurve wird dabei kryptografisch sicher sein und dafür eine Reihe von Bedingungen
    /// erfüllen:
    /// TODO
    pub fn new(n: i32, modul_width: u32, miller_rabin_iterations: u32) -> Self {
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

impl FiniteFieldEllipticCurve {
    ///
    /// Erstellt eine elliptische Kurve nach dem Muster:
    /// y^2 = x^3 - n^2 * x + 0 (mod p)
    ///
    /// Das b der Kurve ist hier bewusst als 0 gewählt und das n wird erst quadriert und dann negiert.
    pub fn new(a: i32, p: BigInt) -> Self {
        let a = a.pow(2).neg();
        Self {
            a,
            b: 0i32,
            prime: p,
        }
    }

    ///
    /// Überprüft, ob ein Punkt auf der elliptischen Kurve liegt.
    ///
    pub fn has_point(&self, point: &FiniteFieldEllipticCurvePoint) -> bool {
        let x_squared = &point.x.pow(2);
        let x_cubed = &point.x * x_squared;
        let y_squared = point.y.pow(2);

        // y^2 = x^3 + ax + b (mod p) ist äquivalent zu (x^3 + ax + b - y^2) % p == 0
        let remainder =
            (x_cubed + &self.a * &point.x + &self.b - y_squared).rem_euclid(&self.prime);
        remainder == BigInt::zero()
    }

    ///
    /// Gibt zurück, ob die Kurve die Bedingung 4a^3 + 27b^2 = 0 erfüllt, also ob die Kurve singulär ist.
    ///
    pub fn is_singular(&self) -> bool {
        let four_a_cubed = 4 * &self.a.pow(3);
        let twenty_seven_b_squared = 27 * &self.b.pow(2);
        (BigInt::from(four_a_cubed) + BigInt::from(twenty_seven_b_squared)).rem_euclid(&self.prime)
            == BigInt::zero()
    }

    pub fn get_order_of_subgroup(&self) -> BigInt {
        self.prime.clone() // TODO Noch falsch. Muss korrigiert werden, damit das Schema zuverlässig klappt!
                           // TODO aktuell ist das erstmal eine übergangslösung, die regelmäßig zu falschen Ergebnissen führt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_point_not() {
        let curve = get_educational_curve();
        let point = FiniteFieldEllipticCurvePoint::new(5.into(), 7.into());
        // (5, 7) liegt nicht auf y^2 = x^3 + 7 (mod 17)
        assert!(!curve.has_point(&point));

        let point = FiniteFieldEllipticCurvePoint::new(4.into(), 6.into());
        // (4, 6) liegt nicht auf y^2 = x^3 + 7 (mod 17). Genaugenommen tut es keiner mit x=4.
        assert!(!curve.has_point(&point));
    }

    #[test]
    fn test_has_point() {
        let curve = get_educational_curve();
        let point = FiniteFieldEllipticCurvePoint::new(5.into(), 8.into());
        // (5, 8) liegt auf y^2 = x^3 + 7 (mod 17)
        assert!(curve.has_point(&point));

        let point = FiniteFieldEllipticCurvePoint::new(5.into(), 9.into());
        // (5, 8) liegt auf y^2 = x^3 + 7 (mod 17)
        assert!(curve.has_point(&point));
    }

    #[test]
    fn test_is_not_singular() {
        let curve = get_educational_curve();
        // 4 * 0^3 + 27 * 7^2 = 0 + 1323 = 1323 (mod 17)= 14 != 0
        assert!(!curve.is_singular());
    }

    #[test]
    fn test_is_singular_trivial() {
        let curve = FiniteFieldEllipticCurve::new(0.into(), 17.into());
        // 4 * 0^3 + 27 * 0^2 = 0 + 0 = 0 (mod 17) = 0
        assert!(curve.is_singular());
    }

    #[test]
    fn test_is_singular_non_trivial() {
        let curve = FiniteFieldEllipticCurve::new(-3, 17.into());
        // 4 * (-3)^3 + 27 * 2^2 = -108 + 108 = 0 (mod 17) = 0
        assert!(curve.is_singular());
    }
}
