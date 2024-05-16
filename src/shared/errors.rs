use crate::math_core::ecc::finite_field_elliptic_curve_point::FiniteFieldEllipticCurvePoint;
use crate::math_core::ecc::secure_finite_field_elliptic_curve::SecureFiniteFieldEllipticCurve;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ArithmeticError {
    /// Wird geworfen, wenn eine Zahl kein Inverses hat.
    ///
    /// # Argumente
    /// * `number` - Die Zahl, die kein Inverses hat.
    /// * `modulus` - Das Modulus, zu dem die Zahl kein Inverses hat.
    #[error("No inverse error: number {0} with modulus {1}")]
    NoInverseError(String, String),

    /// Wird geworfen, wenn eine Zahl keinen diskreten Logarithmus hat.
    ///
    /// # Argumente
    /// * `base` - Die Basis, zu der der diskrete Logarithmus nicht existiert.
    /// * `element` - Das Element, zu dem der diskrete Logarithmus nicht existiert.
    #[error("No discrete logarithm error: base {0} with element {1}")]
    NoDiscreteLogarithmError(String, String),
}

#[derive(Debug, Error, Clone)]
pub enum MenezesVanstoneError {
    #[error("n must not be 0, but it is {0}")]
    InvalidNValueError(i32),

    #[error("Modulus width must be greater than 3, but it is {0}")]
    InvalidModulusWidthError(u32),

    #[error("Number system base must be greater than 0, but it is {0}")]
    InvalidNumberSystemBaseError(u32),
}

#[derive(Debug, Error, Clone)]
pub enum EllipticCurveError {
    #[error("Point {0} is not on curve {1}")]
    PointNotOnCurveError(
        FiniteFieldEllipticCurvePoint,
        SecureFiniteFieldEllipticCurve,
    ),
}
