use std::rc::Rc;

use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::Zero;

use crate::encryption::ecc::elliptic_curve::EllipticCurve;
use crate::encryption::math_functions::number_theory::number_theory_service::{NumberTheoryService, NumberTheoryServiceTrait};
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;
use crate::encryption::math_functions::traits::parity::Parity;

///
/// Repräsentiert einen Punkt auf einer elliptischen Kurve.
/// TODO: Handling von Punkten im Unendlichen und im Ursprung verbessern
///
#[derive(Clone, PartialEq, Debug)]
pub struct EcPoint {
    // Die Koordinaten des Punktes
    pub x: BigInt,
    pub y: BigInt,
    // Referenz auf die elliptische Kurve, auf der der Punkt liegt
    pub curve: Rc<EllipticCurve>,
}

// TODO: Arithmetik von der Datenklasse des Punktes trennen
impl EcPoint {
    pub fn new(x: BigInt, y: BigInt, curve: Rc<EllipticCurve>) -> Self {
        Self { x, y, curve }
    }

    ///
    /// Addiert zwei Punkte auf einer elliptischen Kurve.
    /// Die Punkte müssen auf der gleichen elliptischen Kurve liegen.
    /// Gibt None zurück, falls die Punkte nicht auf der gleichen Kurve liegen.
    ///
    pub fn add(&self, other: &Self) -> Option<Self> {
        if self.curve != other.curve {
            return None;
        }

        // Werden identische Punkte addiert, wird die optimierte Verdopplung aufgerufen
        if self == other {
            return Some(self.double());
        }

        // Falls einer der beiden Punkte im Ursprung liegt, ist das Ergebnis der andere Punkt
        if self.x.is_zero() && self.y.is_zero() {
            return Some(other.clone());
        }
        if other.x.is_zero() && other.y.is_zero() {
            return Some(self.clone());
        }

        let p = &self.curve.p;
        let service = NumberTheoryService::new(Fast); // TODO X: Später korrigieren

        // Zähler der Steigung berechnen
        let slope_numer = &other.y - &self.y;

        // Nenner der Steigung berechnen
        let slope_denom = service.modulo_inverse(&(&other.x - &self.x), p).unwrap();

        // Steigung der Geraden durch die beiden Punkte berechnen
        let slope = (slope_numer * slope_denom).rem_euclid(p);

        let x_sum = (&slope * &slope - &self.x - &other.x).rem_euclid(p);
        let y_sum = (&slope * (&self.x - &x_sum) - &self.y).rem_euclid(p);

        Some(EcPoint::new(x_sum, y_sum, Rc::clone(&self.curve)).normalize())
    }

    ///
    /// Verdoppelt einen Punkt auf einer elliptischen Kurve.
    ///
    pub fn double(&self) -> Self {
        let p = &self.curve.p;
        let service = NumberTheoryService::new(Fast); // TODO X: Später korrigieren

        // Zähler der Steigung berechnen
        let slope_numer = 3u32 * &self.x * &self.x + &self.curve.a;
        // Nenner der Steigung berechnen
        let slope_denom = 2u32 * &self.y;
        let slope_denom = service.modulo_inverse(&slope_denom, p).unwrap();
        // Steigung der Geraden durch die beiden Punkte berechnen
        let slope = (slope_numer * slope_denom).rem_euclid(p);

        let x_sum = (&slope * &slope - 2u32 * &self.x).rem_euclid(p);
        let y_sum = (&slope * (&self.x - &x_sum) - &self.y).rem_euclid(p);

        EcPoint::new(x_sum, y_sum, Rc::clone(&self.curve)).normalize()
    }

    ///
    /// Multipliziert einen Punkt mit einem Skalar.
    /// Dabei wird die optimierte Berechnung in Form des Double-and-add Algorithmus verwendet.
    /// Bei Multiplikation mit 0 wird der Punkt im Ursprung mit Bezug auf die ursprüngliche Kurve
    /// zurückgegeben.
    ///
    pub fn multiply(&self, scalar: &BigInt) -> Self {
        if scalar.is_zero() {
            return EcPoint::new(BigInt::zero(), BigInt::zero(), Rc::clone(&self.curve));
        }
        let mut result = EcPoint::new(BigInt::zero(), BigInt::zero(), Rc::clone(&self.curve));
        let mut addend = self.clone();
        let mut n = scalar.clone();
        while n > BigInt::zero() {
            if n.is_odd() {
                result = result.add(&addend).unwrap();
            }
            addend = addend.double();
            n = n >> 1;
        }
        result
    }

    ///
    /// Normalisiert den Punkt, indem negative Koordinaten in positive Koordinaten umgewandelt werden.
    ///
    fn normalize(&self) -> Self {
        let p = &self.curve.p;
        let mut x = self.x.clone();
        let mut y = self.y.clone();

        // Ggf muss hier mal ein while statt einem if hin, um "vielfach zu tiefe" Zahlen abzufangen?
        if self.x < BigInt::zero() {
            x += p;
        }
        // Ggf muss hier mal ein while statt einem if hin, um "vielfach zu tiefe" Zahlen abzufangen?
        if self.y < BigInt::zero() {
            y += p;
        }

        EcPoint::new(x, y, Rc::clone(&self.curve))
    }
}

#[cfg(test)]
mod tests {
    use crate::encryption::ecc::elliptic_curve::{get_educational_curve_rc, EllipticCurve};

    use super::*;

    #[test]
    fn test_add_trivial() {
        let curve = get_educational_curve_rc();
        let p1 = EcPoint::new(1.into(), 5.into(), Rc::clone(&curve));
        let p2 = EcPoint::new(5.into(), 9.into(), Rc::clone(&curve));
        let p3 = p1.add(&p2).unwrap();
        let expected = EcPoint::new(12.into(), 1.into(), Rc::clone(&curve));
        assert_eq!(p3, expected);
        let has_point = curve.has_point(&p3);
        assert!(has_point);

        let p1 = EcPoint::new(1.into(), 5.into(), Rc::clone(&curve));
        let p2 = EcPoint::new(2.into(), 10.into(), Rc::clone(&curve));
        let p3 = p1.add(&p2).unwrap();
        let expected = EcPoint::new(5.into(), 9.into(), Rc::clone(&curve));
        assert_eq!(p3, expected);
        let has_point = curve.has_point(&p3);
        assert!(has_point);
    }

    #[test]
    fn test_add_from_different_curves_gives_none() {
        let curve1 = get_educational_curve_rc();
        let curve2 = Rc::new(EllipticCurve::new((-5).into(), 1.into(), 19.into()));
        let p1 = EcPoint::new(1.into(), 5.into(), Rc::clone(&curve1));
        let p2 = EcPoint::new(5.into(), 9.into(), Rc::clone(&curve2));
        assert_eq!(p1.add(&p2), None);
    }

    #[test]
    fn test_add_double_point() {
        let curve = get_educational_curve_rc();
        let p1 = EcPoint::new(1.into(), 5.into(), Rc::clone(&curve));
        let p2 = p1.add(&p1).unwrap();
        let p3 = p1.double();
        // A + A und A.double() sollten das gleiche Ergebnis liefern
        assert_eq!(p2, p3);
        // 2 * (1, 5) = (2, 10)
        let expected = EcPoint::new(2.into(), 10.into(), Rc::clone(&curve));
        assert_eq!(p2, expected);
        let has_point = curve.has_point(&p2);
        assert!(has_point);

        let p1 = EcPoint::new(6.into(), 6.into(), Rc::clone(&curve));
        let p2 = p1.add(&p1).unwrap();
        let p3 = p1.double();
        // A + A und A.double() sollten das gleiche Ergebnis liefern
        assert_eq!(p2, p3);
        // 2 * (6, 6) = (1, 5)
        let expected = EcPoint::new(1.into(), 5.into(), Rc::clone(&curve));
        assert_eq!(p3, expected);
        let has_point = curve.has_point(&p3);
        assert!(has_point);

        let p1 = EcPoint::new(15.into(), 4.into(), Rc::clone(&curve));
        let p2 = p1.add(&p1).unwrap();
        let p3 = p1.double();
        // A + A und A.double() sollten das gleiche Ergebnis liefern
        assert_eq!(p2, p3);
        // 2 * (6, 6) = (1, 5)
        let expected = EcPoint::new(2.into(), 7.into(), Rc::clone(&curve));
        assert_eq!(p3, expected);
        let has_point = curve.has_point(&p3);
        assert!(has_point);
    }

    #[test]
    fn test_multiply_trivial() {
        let curve = get_educational_curve_rc();
        let p1 = EcPoint::new(12.into(), 16.into(), Rc::clone(&curve));
        let p2 = p1.multiply(&1.into());
        assert_eq!(p1, p2);

        let p3 = p1.multiply(&2.into());
        let expected = EcPoint::new(1.into(), 5.into(), Rc::clone(&curve));
        assert_eq!(p3, expected);

        let p4 = p1.multiply(&8.into());
        let expected = EcPoint::new(12.into(), 1.into(), Rc::clone(&curve));
        assert_eq!(p4, expected);

        let p5 = p1.multiply(&14.into());
        let expected = EcPoint::new(2.into(), 7.into(), curve);
        assert_eq!(p5, expected);
    }

    #[test]
    fn test_multiply_with_zero() {
        let curve = get_educational_curve_rc();
        let p1 = EcPoint::new(12.into(), 16.into(), Rc::clone(&curve));
        let p2 = p1.multiply(&0.into());
        let expected = EcPoint::new(BigInt::zero(), BigInt::zero(), curve);
        assert_eq!(p2, expected);
    }
}
