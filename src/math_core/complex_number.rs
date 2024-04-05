use bigdecimal::num_bigint::BigInt;
use bigdecimal::{BigDecimal, Signed, Zero};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug, PartialEq)]
pub struct ComplexNumber {
    pub real: BigDecimal,
    pub imaginary: BigDecimal,
}

impl ComplexNumber {
    pub fn new(real: BigDecimal, imaginary: BigDecimal) -> Self {
        Self { real, imaginary }
    }

    pub fn conjugate(&self) -> ComplexNumber {
        ComplexNumber::new(self.real.clone(), -self.imaginary.clone())
    }

    pub fn negate(&self) -> ComplexNumber {
        ComplexNumber::new(-self.real.clone(), -self.imaginary.clone())
    }

    pub fn is_in_first_quadrant(&self) -> bool {
        self.real.is_positive() && self.imaginary.is_positive()
    }

    pub fn is_in_third_quadrant(&self) -> bool {
        self.real.is_negative() && self.imaginary.is_negative()
    }

    pub fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imaginary.is_zero()
    }

    pub fn gaussian_integer(self) -> Self {
        Self {
            real: self.real.round(0),
            imaginary: self.imaginary.round(0),
        }
    }
}

pub fn complex_euclidean_algorithm(a: ComplexNumber, b: ComplexNumber) -> ComplexNumber {

    let mut g = a;
    let mut g_prev = b;

    while !g.is_zero() {
        let tmp = g.clone();
        g = &g_prev - &(&g * &(&g_prev / &g).gaussian_integer());
        g_prev = tmp.clone();
    }
    ComplexNumber {
        real: g_prev.real.clone(),
        imaginary: g_prev.imaginary.clone(),
    }
}

impl Add for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: &self.real + &rhs.real,
            imaginary: &self.imaginary + &rhs.imaginary,
        }
    }
}

impl Add for &ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: &self.real + &rhs.real,
            imaginary: &self.imaginary + &rhs.imaginary,
        }
    }
}

impl Sub for ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: &self.real - &rhs.real,
            imaginary: &self.imaginary - &rhs.imaginary,
        }
    }
}

impl Sub for &ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: &self.real - &rhs.real,
            imaginary: &self.imaginary - &rhs.imaginary,
        }
    }
}

impl Mul for ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: &self.real * &rhs.real - &self.imaginary * &rhs.imaginary,
            imaginary: &self.real * &rhs.imaginary + &self.imaginary * &rhs.real,
        }
    }
}

impl Mul for &ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: &self.real * &rhs.real - &self.imaginary * &rhs.imaginary,
            imaginary: &self.real * &rhs.imaginary + &self.imaginary * &rhs.real,
        }
    }
}

impl Div for ComplexNumber {
    type Output = ComplexNumber;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            real: (&self.real * &rhs.real + &self.imaginary * &rhs.imaginary)
                / (&rhs.real * &rhs.real + &rhs.imaginary * &rhs.imaginary),
            imaginary: (&self.imaginary * &rhs.real - &self.real * &rhs.imaginary)
                / (&rhs.real * &rhs.real + &rhs.imaginary * &rhs.imaginary),
        }
    }
}

impl Div for &ComplexNumber {
    type Output = ComplexNumber;

    fn div(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: (&self.real * &rhs.real + &self.imaginary * &rhs.imaginary)
                / (&rhs.real * &rhs.real + &rhs.imaginary * &rhs.imaginary),
            imaginary: (&self.imaginary * &rhs.real - &self.real * &rhs.imaginary)
                / (&rhs.real * &rhs.real + &rhs.imaginary * &rhs.imaginary),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_test() {
        let x = ComplexNumber {
            real: BigDecimal::from(-6),
            imaginary: BigDecimal::from(17),
        };
        let y = ComplexNumber {
            real: BigDecimal::from(3),
            imaginary: BigDecimal::from(4),
        };
        assert_eq!(complex_euclidean_algorithm(y.clone(), x.clone()), y);
        assert_eq!(complex_euclidean_algorithm(x.clone(), y.clone()), y);
    }
}
