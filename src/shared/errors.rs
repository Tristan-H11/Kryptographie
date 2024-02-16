use std::fmt;

#[derive(Debug, Clone)]
pub enum ArithmeticError {
    /// Wird geworfen, wenn eine Zahl kein Inverses hat.
    ///
    /// # Argumente
    /// * `number` - Die Zahl, die kein Inverses hat.
    /// * `modulus` - Das Modulus, zu dem die Zahl kein Inverses hat.
    NoInverseError(String, String),

    /// Wird geworfen, wenn eine Zahl keinen diskreten Logarithmus hat.
    ///
    /// # Argumente
    /// * `base` - Die Basis, zu der der diskrete Logarithmus nicht existiert.
    /// * `element` - Das Element, zu dem der diskrete Logarithmus nicht existiert.
    NoDiscreteLogarithmError(String, String),
}

impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithmeticError::NoInverseError(number, modulus) => write!(
                f,
                "No inverse error: number {} with modulus {}",
                number, modulus
            ),
            ArithmeticError::NoDiscreteLogarithmError(base, element) => write!(
                f,
                "No discrete logarithm error: base {} with element {}",
                base, element
            ),
        }
    }
}

impl std::error::Error for ArithmeticError {}

#[derive(Debug, Clone)]
pub enum RsaError {
    /// Wird geworfen, wenn die Schlüsselerzeugung fehlschlägt.
    #[allow(dead_code)] // TODO: Wieder einbauen
    KeyGenerationError,
}

impl fmt::Display for RsaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RsaError::KeyGenerationError => write!(f, "RSA Key generation error"),
        }
    }
}
