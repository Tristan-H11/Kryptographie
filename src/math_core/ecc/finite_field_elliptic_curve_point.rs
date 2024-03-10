use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::Zero;

use crate::math_core::ecc::finite_field_elliptic_curve::FiniteFieldEllipticCurve;
use crate::math_core::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::traits::parity::Parity;

///
/// Repräsentiert einen Punkt auf einer elliptischen Kurve.
/// Die Koordinaten des Punktes sind Elemente eines endlichen Körpers.
/// TODO: Handling von Punkten im Unendlichen und im Ursprung verbessern
///
#[derive(Clone, PartialEq, Debug)]
pub struct FiniteFieldEllipticCurvePoint {
    // Die Koordinaten des Punktes
    pub x: BigInt,
    pub y: BigInt,
}

// TODO: Arithmetik von der Datenklasse des Punktes trennen, dabei auch die Kurve aus dem Punkt entfernen.
impl FiniteFieldEllipticCurvePoint {
    pub fn new(x: BigInt, y: BigInt) -> Self {
        Self { x, y }
    }

    /// Addiert zwei nicht-identische (!) Punkte auf einer elliptischen Kurve.
    /// Die Punkte müssen auf der gleichen elliptischen Kurve liegen.
    /// Sind sie identisch, kann die Berechnung fehlschlagen!
    pub fn add(&self, other: &Self, prime: &BigInt) -> Self {
        // Falls einer der beiden Punkte im Ursprung liegt, ist das Ergebnis der andere Punkt
        if self.x.is_zero() && self.y.is_zero() {
            return other.clone();
        }
        if other.x.is_zero() && other.y.is_zero() {
            return self.clone();
        }

        let service = NumberTheoryService::new(Fast); // TODO X: Später korrigieren

        // Zähler der Steigung berechnen
        let slope_numer = &other.y - &self.y;

        // Nenner der Steigung berechnen
        let slope_denom = service
            .modulo_inverse(&(&other.x - &self.x), prime)
            .unwrap();

        // Steigung der Geraden durch die beiden Punkte berechnen
        let slope = slope_numer * slope_denom;

        let x_sum = &slope * &slope - &self.x - &other.x;
        let y_sum = &slope * (&self.x - &x_sum) - &self.y;

        FiniteFieldEllipticCurvePoint::new(x_sum, y_sum).normalize(prime)
    }

    ///
    /// Verdoppelt einen Punkt auf einer elliptischen Kurve.
    ///
    pub fn double(&self, curve: &FiniteFieldEllipticCurve) -> Self {
        let service = NumberTheoryService::new(Fast); // TODO X: Später korrigieren
        let p = &curve.prime;
        // Zähler der Steigung berechnen
        let slope_numer = 3u32 * &self.x * &self.x + &curve.a;
        // Nenner der Steigung berechnen
        let slope_denom = 2u32 * &self.y;
        let slope_denom = service.modulo_inverse(&slope_denom, p).unwrap();
        // Steigung der Geraden durch die beiden Punkte berechnen
        let slope = slope_numer * slope_denom;

        let x_sum = &slope * &slope - 2u32 * &self.x;
        let y_sum = &slope * (&self.x - &x_sum) - &self.y;

        FiniteFieldEllipticCurvePoint::new(x_sum, y_sum).normalize(p)
    }

    ///
    /// Multipliziert einen Punkt mit einem Skalar.
    /// Dabei wird die optimierte Berechnung in Form des Double-and-add Algorithmus verwendet.
    /// Bei Multiplikation mit 0 wird der Punkt im Ursprung mit Bezug auf die ursprüngliche Kurve
    /// zurückgegeben.
    ///
    pub fn multiply(&self, scalar: &BigInt, curve: &FiniteFieldEllipticCurve) -> Self {
        if scalar.is_zero() {
            return FiniteFieldEllipticCurvePoint::new(BigInt::zero(), BigInt::zero());
        }
        let mut result = FiniteFieldEllipticCurvePoint::new(BigInt::zero(), BigInt::zero());
        let mut addend = self.clone();
        let mut n = scalar.clone();
        while n > BigInt::zero() {
            if n.is_odd() {
                result = result.add(&addend, &curve.prime);
            }
            addend = addend.double(curve);
            n = n >> 1;
        }
        result
    }

    ///
    /// Normalisiert den Punkt, indem negative Koordinaten in positive Koordinaten umgewandelt werden.
    /// Anschließend wird der Punkt wieder in den Körper der elliptischen Kurve zurückgeführt.
    ///
    fn normalize(&self, prime: &BigInt) -> Self {
        let mut x = self.x.clone();
        let mut y = self.y.clone();

        // Ggf muss hier mal ein while statt einem if hin, um "vielfach zu tiefe" Zahlen abzufangen?
        if self.x < BigInt::zero() {
            x += prime;
        }
        // Ggf muss hier mal ein while statt einem if hin, um "vielfach zu tiefe" Zahlen abzufangen?
        if self.y < BigInt::zero() {
            y += prime;
        }

        let normalized_x = x.rem_euclid(prime);
        let normalized_y = y.rem_euclid(prime);
        FiniteFieldEllipticCurvePoint::new(normalized_x, normalized_y)
    }
}

#[cfg(test)]
mod tests {
    use crate::math_core::ecc::finite_field_elliptic_curve::get_educational_curve;

    use super::*;

    #[test]
    fn test_add_trivial() {
        let curve = get_educational_curve();
        let p1 = FiniteFieldEllipticCurvePoint::new(1.into(), 5.into());
        let p2 = FiniteFieldEllipticCurvePoint::new(5.into(), 9.into());
        let p3 = p1.add(&p2, &curve.prime);
        let expected = FiniteFieldEllipticCurvePoint::new(12.into(), 1.into());
        assert_eq!(p3, expected);
        let has_point = curve.has_point(&p3);
        assert!(has_point);

        let p1 = FiniteFieldEllipticCurvePoint::new(1.into(), 5.into());
        let p2 = FiniteFieldEllipticCurvePoint::new(2.into(), 10.into());
        let p3 = p1.add(&p2, &curve.prime);
        let expected = FiniteFieldEllipticCurvePoint::new(5.into(), 9.into());
        assert_eq!(p3, expected);
        let has_point = curve.has_point(&p3);
        assert!(has_point);
    }

    #[test]
    #[should_panic]
    fn test_identic_points_panics() {
        let curve = &get_educational_curve();
        let p1 = FiniteFieldEllipticCurvePoint::new(1.into(), 5.into());
        let prime = &curve.prime;
        p1.add(&p1, prime);
    }

    #[test]
    fn test_multiply_trivial() {
        let curve = &get_educational_curve();
        let p1 = FiniteFieldEllipticCurvePoint::new(12.into(), 16.into());
        let p2 = p1.multiply(&1.into(), curve);
        assert_eq!(p1, p2);

        let p3 = p1.multiply(&2.into(), curve);
        let expected = FiniteFieldEllipticCurvePoint::new(1.into(), 5.into());
        assert_eq!(p3, expected);

        let p4 = p1.multiply(&8.into(), curve);
        let expected = FiniteFieldEllipticCurvePoint::new(12.into(), 1.into());
        assert_eq!(p4, expected);

        let p5 = p1.multiply(&14.into(), curve);
        let expected = FiniteFieldEllipticCurvePoint::new(2.into(), 7.into());
        assert_eq!(p5, expected);
    }

    #[test]
    fn test_multiply_with_zero() {
        let curve = get_educational_curve();
        let p1 = FiniteFieldEllipticCurvePoint::new(12.into(), 16.into());
        let p2 = p1.multiply(&0.into(), &curve);
        let expected = FiniteFieldEllipticCurvePoint::new(BigInt::zero(), BigInt::zero());
        assert_eq!(p2, expected);
    }
}
