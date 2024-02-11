use bigdecimal::num_bigint::BigInt;
use bigdecimal::Zero;
use crate::encryption::ecc::point::Point;

///
/// Repräsentiert eine elliptische Kurve.
///
/// Die Koeffizienten a und b der elliptischen Kurve sind die Koeffizienten der Gleichung
/// y^2 = x^3 + ax + b, die die elliptische Kurve definiert.
/// Um die Kurve über einem endlichen Körper zu definieren, wird auch der Modulus p benötigt.
///
pub struct EllipticCurve {
    // Die Koeffizienten a und b der elliptischen Kurve
    pub a: BigInt,
    pub b: BigInt,
    // Der Modulus p der elliptischen Kurve, um sie über einem endlichen Körper zu definieren
    pub p: BigInt,
}

impl EllipticCurve {
    pub fn new(a: BigInt, b: BigInt, p: BigInt) -> Self {
        Self { a, b, p }
    }

    ///
    /// Überprüft, ob ein Punkt auf der elliptischen Kurve liegt.
    ///
    pub fn has_point(&self, point: &Point) -> bool {
        let x_squared = &point.x.pow(2);
        let x_cubed = &point.x * x_squared;
        let y_squared = point.y.pow(2);

        // y^2 = x^3 + ax + b (mod p) ist äquivalent zu (x^3 + ax + b - y^2) % p == 0
        let remainder = (x_cubed + &self.a * &point.x + &self.b - y_squared) % &self.p;
        remainder == BigInt::zero()
    }

    ///
    /// Gibt zurück, ob die Kurve die Bedingung 4a^3 + 27b^2 != 0 erfüllt, also ob die Kurve singulär ist.
    ///
    pub fn is_singular(&self) -> bool {
        let four_a_cubed = 4u32 * &self.a.pow(3);
        let twenty_seven_b_squared = 27u32 * &self.b.pow(2);
        (four_a_cubed + twenty_seven_b_squared) % &self.p == BigInt::zero()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn get_educational_curve() -> EllipticCurve {
        let p = BigInt::from(17u32);
        let a = BigInt::from(0u32);
        let b = BigInt::from(7u32);
        EllipticCurve::new(a, b, p)
    }

    #[test]
    fn test_has_point_not() {
        let curve = get_educational_curve();
        let point = Point::new(
            BigInt::from(5u32),
            BigInt::from(7u32),
        );
        // (5, 7) liegt nicht auf y^2 = x^3 + 7 (mod 17)
        assert!(!curve.has_point(&point));

        let point = Point::new(
            BigInt::from(4u32),
            BigInt::from(6u32),
        );
        // (4, 6) liegt nicht auf y^2 = x^3 + 7 (mod 17). Genaugenommen tut es keiner mit x=4.
        assert!(!curve.has_point(&point));
    }

    #[test]
    fn test_has_point() {
        let curve = get_educational_curve();
        let point = Point::new(
            BigInt::from(5u32),
            BigInt::from(8u32),
        );
        // (5, 8) liegt auf y^2 = x^3 + 7 (mod 17)
        assert!(curve.has_point(&point));

        let point = Point::new(
            BigInt::from(5u32),
            BigInt::from(9u32),
        );
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
        let curve = EllipticCurve::new(
            BigInt::from(0u32),
            BigInt::from(0u32),
            BigInt::from(17u32),
        );
        // 4 * 0^3 + 27 * 0^2 = 0 + 0 = 0 (mod 17) = 0
        assert!(curve.is_singular());
    }

    #[test]
    fn test_is_singular_non_trivial() {
        let curve = EllipticCurve::new(
            BigInt::from(-3),
            2.into(),
            17.into(),
        );
        // 4 * (-3)^3 + 27 * 2^2 = -108 + 108 = 0 (mod 17) = 0
        assert!(curve.is_singular());
    }
}