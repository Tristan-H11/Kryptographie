use std::rc::Rc;

use bigdecimal::num_bigint::BigInt;
use bigdecimal::num_traits::Euclid;
use bigdecimal::Zero;

use crate::encryption::ecc::elliptic_curve::EllipticCurve;
use crate::encryption::math_functions::number_theory::number_theory_service::{NumberTheoryService, NumberTheoryServiceTrait};
use crate::encryption::math_functions::number_theory::number_theory_service::NumberTheoryServiceSpeed::Fast;

///
/// Repräsentiert einen Punkt auf einer elliptischen Kurve.
///
#[derive(Clone, PartialEq, Debug)]
pub struct EcPoint {
    // Die Koordinaten des Punktes
    pub x: BigInt,
    pub y: BigInt,
    // Statische Referenz auf die elliptische Kurve, auf der der Punkt liegt
    pub curve: Rc<EllipticCurve>,
}

impl EcPoint {
    pub fn new(x: BigInt, y: BigInt, curve: Rc<EllipticCurve>) -> Self {
        Self { x, y, curve }
    }

    ///
    /// Addiert zwei Punkte auf einer elliptischen Kurve.
    /// Die Punkte müssen auf der gleichen elliptischen Kurve liegen.
    /// TODO: Verdopplung eines Punktes auslagern ala "if equals then call double else add"
    ///
    fn add(self, other: Self) -> Self {
        // TODO X: Auf Result umstellen, bezüglich des Asserts
        assert_eq!(
            self.curve, other.curve,
            "Die Punkte müssen auf der gleichen elliptischen Kurve liegen."
        );

        // Falls einer der beiden Punkte im Ursprung liegt, ist das Ergebnis der andere Punkt
        if self.x.is_zero() && self.y.is_zero() {
            return other.clone();
        }
        if other.x.is_zero() && other.y.is_zero() {
            return self.clone();
        }

        let p = &self.curve.p;
        let service = NumberTheoryService::new(Fast); // TODO X: Später korrigieren

        // Zähler der Steigung berechnen
        let slope_numer = if self.x == other.x {
            3u32 * &self.x * &self.x + &self.curve.a
        } else {
            &other.y - &self.y
        };
        // Nenner der Steigung berechnen
        let slope_denom = if self.x == other.x {
            let product = 2u32 * &self.y;
            service.modulo_inverse(&product, p).unwrap()
        } else {
            let diff = &other.x - &self.x;
            service.modulo_inverse(&diff, p).unwrap()
        };
        // Steigung der Geraden durch die beiden Punkte berechnen
        let slope = (slope_numer * slope_denom).rem_euclid(p);

        let x_sum = (&slope * &slope - &self.x - &other.x).rem_euclid(p);
        let y_sum = (&slope * (&self.x - &x_sum) - &self.y).rem_euclid(p);

        EcPoint::new(x_sum, y_sum, self.curve).normalize()
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
        let p3 = p1.add(p2);
        assert_eq!(p3, EcPoint::new(12.into(), 1.into(), Rc::clone(&curve)));
        let has_point = curve.has_point(&p3);
        assert!(has_point);

        let p1 = EcPoint::new(1.into(), 5.into(), Rc::clone(&curve));
        let p2 = EcPoint::new(2.into(), 10.into(), Rc::clone(&curve));
        let p3 = p1.add(p2);
        assert_eq!(p3, EcPoint::new(5.into(), 9.into(), Rc::clone(&curve)));
        let has_point = curve.has_point(&p3);
        assert!(has_point);
    }

    #[test]
    #[should_panic]
    fn test_add_from_different_curves_should_panic() {
        let curve1 = get_educational_curve_rc();
        let curve2 = Rc::new(EllipticCurve::new((-5).into(), 1.into(), 19.into()));
        let p1 = EcPoint::new(1.into(), 5.into(), Rc::clone(&curve1));
        let p2 = EcPoint::new(5.into(), 9.into(), Rc::clone(&curve2));
        p1.add(p2);
    }

    #[test]
    fn test_add_double_point() {
        let curve = get_educational_curve_rc();
        let p1 = EcPoint::new(1.into(), 5.into(), Rc::clone(&curve));
        let p2 = p1.clone();
        let p3 = p1.add(p2);
        // 2 * (1, 5) = (2, 10)
        assert_eq!(p3, EcPoint::new(2.into(), 10.into(), Rc::clone(&curve)));
        let has_point = curve.has_point(&p3);
        assert!(has_point);

        let p1 = EcPoint::new(6.into(), 6.into(), Rc::clone(&curve));
        let p2 = p1.clone();
        let p3 = p1.add(p2);
        // 2 * (6, 6) = (1, 5)
        assert_eq!(p3, EcPoint::new(1.into(), 5.into(), Rc::clone(&curve)));
        let has_point = curve.has_point(&p3);
        assert!(has_point);

        let p1 = EcPoint::new(15.into(), 4.into(), Rc::clone(&curve));
        let p2 = p1.clone();
        let p3 = p1.add(p2);
        // 2 * (6, 6) = (1, 5)
        assert_eq!(p3, EcPoint::new(2.into(), 7.into(), Rc::clone(&curve)));
        let has_point = curve.has_point(&p3);
        assert!(has_point);
    }
}
