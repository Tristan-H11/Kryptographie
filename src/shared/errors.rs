use std::fmt;

#[derive(Debug, Clone)]
pub enum ArithmeticError {
    /// Wird geworfen, wenn eine Zahl kein Inverses hat.
    ///
    /// # Argumente
    /// * `number` - Die Zahl, die kein Inverses hat.
    /// * `modulus` - Das Modulus, zu dem die Zahl kein Inverses hat.
    NoInverseError(String, String),
}

impl fmt::Display for ArithmeticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArithmeticError::NoInverseError(number, modulus) => write!(f, "No inverse error: number {} with modulus {}", number, modulus),
        }
    }
}