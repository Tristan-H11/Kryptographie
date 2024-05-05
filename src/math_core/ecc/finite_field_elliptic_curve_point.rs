use crate::api::endpoints::mv::EcPointBean;
use anyhow::{bail, Context, Result};
use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::{One, Zero};
use std::ops::Add;

use crate::math_core::ecc::secure_finite_field_elliptic_curve::SecureFiniteFieldEllipticCurve;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::traits::parity::Parity;

/// Repräsentiert einen Punkt auf einer elliptischen Kurve.
/// Die Koordinaten des Punktes sind Elemente eines endlichen Körpers.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct FiniteFieldEllipticCurvePoint {
    // Die Koordinaten des Punktes
    pub x: BigInt,
    pub y: BigInt,
    pub is_infinite: bool,
}

impl From<EcPointBean> for FiniteFieldEllipticCurvePoint {
    /// Mapped die Bean in das Domain-Modell
    fn from(point: EcPointBean) -> Self {
        Self {
            x: point.x.parse().unwrap(), // TODO .unwrap() durch turbofish ersetzen ? --> .parse()::<BigFloat>()
            y: point.y.parse().unwrap(), // TODO .unwrap() durch turbofish ersetzen ? --> .parse()::<BigFloat>()
            is_infinite: point.is_infinite,
        }
    }
}

impl FiniteFieldEllipticCurvePoint {
    pub fn new(x: BigInt, y: BigInt) -> Self {
        Self {
            x,
            y,
            is_infinite: false,
        }
    }

    pub fn infinite() -> Self {
        Self {
            x: BigInt::zero(),
            y: BigInt::zero(),
            is_infinite: true,
        }
    }

    /// Addiert zwei Punkte auf einer elliptischen Kurve.
    /// Die Punkte müssen auf der gleichen elliptischen Kurve liegen.
    pub fn add(&self, other: &Self, curve: &SecureFiniteFieldEllipticCurve) -> Result<Self> {
        // Liegen die Punkte nicht auf der gleichen Kurve, ist das Ergebnis undefiniert.
        if !curve.has_point(self) || !curve.has_point(other) {
            bail!(
                "Points are not on the curve. P1: {:#?}, P2: {:#?}, Curve: {:#?}",
                self,
                other,
                curve
            )
        }

        // Liegt einer der beiden Punkte im Unendlichen, so ist das Ergebnis der je andere Punkt.
        if self.is_infinite {
            return Ok(other.clone());
        }
        if other.is_infinite {
            return Ok(self.clone());
        }

        // Negieren sich zwei Punkte, so erhält man ebenfalls den Punkt im Unendlichen.
        if self.x == other.x && (&self.y).add(&other.y).rem_euclid(&curve.prime).is_zero() {
            return Ok(FiniteFieldEllipticCurvePoint::infinite());
        }

        // Handelt es sich um identische Punkte, so wird der Punkt verdoppelt.
        if self == other {
            return Ok(self.double(curve));
        }

        let prime = &curve.prime;

        let service = NumberTheoryService::new(Fast); // TODO X: Später korrigieren

        // Zähler der Steigung berechnen
        let slope_numer = &other.y - &self.y;

        // Nenner der Steigung berechnen
        let slope_denom = service
            .modulo_inverse(&(&other.x - &self.x), prime)
            .unwrap();

        // Steigung der Geraden durch die beiden Punkte berechnen
        let slope = slope_numer * slope_denom;

        let x_sum = &slope.pow(2) - &self.x - &other.x;
        // Version aus Skript S. 57 ohne Verteilen des Negativzeichens: let y_sum = - (&slope * (&x_sum - &self.x)) - &self.y;
        let y_sum = &slope * (&self.x - &x_sum) - &self.y;

        Ok(FiniteFieldEllipticCurvePoint::new(x_sum, y_sum).normalize(prime))
    }

    /// Verdoppelt einen Punkt auf einer elliptischen Kurve.
    pub fn double(&self, curve: &SecureFiniteFieldEllipticCurve) -> Self {
        if self.is_infinite {
            return self.clone();
        }
        // Bei der Verdopplung wird anhand der Tangente gerechnet.
        // Ist die Y-Koordinate 0, so ist sie senkrecht und der resultierende Punkt im Unendlichen.
        if self.y.is_zero() {
            return FiniteFieldEllipticCurvePoint::infinite();
        }
        let service = NumberTheoryService::new(Fast); // TODO X: Später korrigieren
        let p = &curve.prime;
        // Zähler der Steigung berechnen
        let slope_numer = 3 * (&self.x).pow(2) + &curve.a;
        // Nenner der Steigung berechnen
        let slope_denom = 2 * &self.y;
        let slope_denom = service.modulo_inverse(&slope_denom, p).unwrap();
        // Steigung der Geraden durch die beiden Punkte berechnen
        let slope = slope_numer * slope_denom;

        let x_sum = &slope * &slope - 2 * &self.x;
        let y_sum = &slope * (&self.x - &x_sum) - &self.y;

        FiniteFieldEllipticCurvePoint::new(x_sum, y_sum).normalize(p)
    }

    /// Multipliziert einen Punkt mit einem Skalar.
    /// Dabei wird die optimierte Berechnung in Form des Double-and-add Algorithmus verwendet.
    /// Bei Multiplikation mit 0 wird der Punkt im Ursprung mit Bezug auf die ursprüngliche Kurve
    /// zurückgegeben.
    pub fn multiply(
        &self,
        scalar: &BigInt,
        curve: &SecureFiniteFieldEllipticCurve,
    ) -> Result<Self> {
        // Bei einer 1 passiert nichts
        if scalar.is_one() {
            return Ok(self.clone());
        }
        // Bei einer 2 wird verdoppelt
        if scalar == &BigInt::from(2) {
            return Ok(self.double(curve));
        }
        // Ist der Punkt der Generator und der Skalar die Ordnung des Generators, wird der Punkt
        // im Unendlichen zurückgegeben.
        if self == &curve.generator && scalar == &curve.order_of_subgroup {
            return Ok(FiniteFieldEllipticCurvePoint::infinite());
        }

        let mut result = FiniteFieldEllipticCurvePoint::infinite();
        let mut addend = self.clone();
        let mut n = scalar.clone();
        while n > BigInt::zero() {
            if n.is_odd() {
                result = result
                    .add(&addend, &curve)
                    .context("Error while adding point in multiply operation")?;
            }
            addend = addend.double(curve);
            n = n >> 1;
        }
        Ok(result)
    }

    ///
    /// Normalisiert den Punkt, indem negative Koordinaten in positive Koordinaten umgewandelt werden.
    /// Anschließend wird der Punkt wieder in den Körper der elliptischen Kurve zurückgeführt.
    ///
    fn normalize(&self, prime: &BigInt) -> Self {
        if self.is_infinite {
            return self.clone();
        }

        let mut x = self.x.clone();
        let mut y = self.y.clone();

        // Ggf muss hier mal ein while statt einem if hin, um "vielfach zu tiefe" Zahlen abzufangen? -- NEIN, wenn while, dann endloser loop
        if x < BigInt::zero() {
          x += prime;
        }
        if y < BigInt::zero() {
          y += prime;
        }

        let normalized_x = x.rem_euclid(prime);
        let normalized_y = y.rem_euclid(prime);
        FiniteFieldEllipticCurvePoint::new(normalized_x, normalized_y)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_curve() -> SecureFiniteFieldEllipticCurve {
        SecureFiniteFieldEllipticCurve::new(5, 16, 40).unwrap()
    }

    #[test]
    fn test_add_trivial() {
        let curve = get_curve();
        assert!(curve.order_of_subgroup > 8.into());
        let p1 = curve.generator.clone();
        let p2 = curve.generator.multiply(&4.into(), &curve).unwrap();
        let p3 = p1.add(&p2, &curve).unwrap();
        let expected = p1.multiply(&5.into(), &curve).unwrap();
        assert_eq!(p3, expected);
        let has_point = curve.has_point(&p3);
        assert!(has_point, "{:?}, {:?}", p3, curve);
    }

    #[test]
    fn test_add_identical_points_doubles() {
        let curve = get_curve();
        let p1 = curve.generator.clone();
        let result = p1.add(&p1, &curve).unwrap();
        assert_eq!(result, p1.double(&curve));
    }

    #[test]
    fn test_multiply_trivial() {
        let curve = get_curve();
        let p1 = curve.generator.clone();
        let identical = p1.multiply(&1.into(), &curve).unwrap();
        assert_eq!(p1, identical);

        let doubled = p1.multiply(&2.into(), &curve).unwrap();
        let expected = curve.generator.double(&curve);
        assert_eq!(doubled, expected);

        let p2 = doubled.multiply(&8.into(), &curve).unwrap();
        let expected = curve.generator.multiply(&16.into(), &curve).unwrap();
        assert_eq!(p2, expected);
    }

    #[test]
    fn test_multiply_by_order_gives_infinity() {
        let curve = get_curve();
        let p1 = curve.generator.clone();
        let p2 = p1.multiply(&curve.order_of_subgroup, &curve).unwrap();
        let expected = FiniteFieldEllipticCurvePoint::infinite();
        assert_eq!(p2, expected);
    }

    #[test]
    fn test_multiply_with_zero() {
        let curve = get_curve();
        let p2 = curve.generator.multiply(&0.into(), &curve).unwrap();
        let expected = FiniteFieldEllipticCurvePoint::infinite();
        assert_eq!(p2, expected);
    }

    #[test]
    fn test_add_with_infinity() {
        let curve = get_curve();
        let generator = curve.generator.clone();
        let infinity = FiniteFieldEllipticCurvePoint::infinite();

        // Point + 0 = Point
        let p2 = generator.add(&infinity, &curve).unwrap();
        assert_eq!(p2, generator);

        // 0 + Point = Point
        let p3 = infinity.add(&generator, &curve).unwrap();
        assert_eq!(p3, generator);
    }
}
