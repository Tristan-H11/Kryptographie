use bigdecimal::num_bigint::BigInt;
use bigdecimal::Signed;

#[derive(Clone, Debug, PartialEq)]
pub struct ComplexNumber {
    pub real: BigInt,
    pub imaginary: BigInt,
}

impl ComplexNumber {
    pub fn new(real: BigInt, imaginary: BigInt) -> Self {
        Self { real, imaginary }
    }

    pub fn conjugate(&self) -> ComplexNumber {
        ComplexNumber::new(self.real.clone(), -self.imaginary.clone())
    }

    pub fn negate(&self) -> ComplexNumber {
        ComplexNumber::new(-self.real.clone(), -self.imaginary.clone())
    }

    pub fn add(&self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber::new(&self.real + &other.real, &self.imaginary + &other.imaginary)
    }

    pub fn subtract(&self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber::new(&self.real - &other.real, &self.imaginary - &other.imaginary)
    }

    pub fn multiply(&self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber::new(
            &self.real * &other.real - &self.imaginary * &other.imaginary,
            &self.real * &other.imaginary + &self.imaginary * &other.real,
        )
    }

    pub fn divide(&self, other: &ComplexNumber) -> ComplexNumber {
        let denominator = &other.real * &other.real + &other.imaginary * &other.imaginary;
        ComplexNumber::new(
            (&self.real * &other.real + &self.imaginary * &other.imaginary) / &denominator,
            (&self.imaginary * &other.real - &self.real * &other.imaginary) / &denominator,
        )
    }

    pub fn is_in_first_quadrant(&self) -> bool {
        self.real.is_positive() && self.imaginary.is_positive()
    }

    pub fn is_in_third_quadrant(&self) -> bool {
        self.real.is_negative() && self.imaginary.is_negative()
    }
}
