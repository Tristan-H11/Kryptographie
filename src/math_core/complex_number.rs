use bigdecimal::num_bigint::ToBigInt;

use bigdecimal::{BigDecimal, Signed, Zero};
use num::BigInt;

use std::ops::{Add, Div, Mul, Sub};

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

    pub fn is_in_first_quadrant(&self) -> bool {
        self.real.is_positive() && self.imaginary.is_positive()
    }

    pub fn is_in_third_quadrant(&self) -> bool {
        self.real.is_negative() && self.imaginary.is_negative()
    }

    pub fn absolute_value(&self) -> Option<BigDecimal> {
        BigDecimal::from(&self.real * &self.real + &self.imaginary * &self.imaginary).sqrt()
    }

    pub fn is_greater_than(&self, other: &Self) -> bool {
        self.absolute_value() > other.absolute_value()
    }

    pub fn is_less_than(&self, other: &Self) -> bool {
        self.absolute_value() < other.absolute_value()
    }

    pub fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imaginary.is_zero()
    }

    pub fn div_round(&self, rhs: &Self) -> Self {
        Self {
            real: (BigDecimal::from(&self.real * &rhs.real + &self.imaginary * &rhs.imaginary)
                / BigDecimal::from(&rhs.real * &rhs.real + &rhs.imaginary * &rhs.imaginary))
            .round(0)
            .to_bigint()
            .unwrap(),
            imaginary: (BigDecimal::from(
                &self.imaginary * &rhs.real - &self.real * &rhs.imaginary,
            ) / BigDecimal::from(
                &rhs.real * &rhs.real + &rhs.imaginary * &rhs.imaginary,
            ))
            .round(0)
            .to_bigint()
            .unwrap(),
        }
    }
}

pub fn complex_euclidean_algorithm(a: ComplexNumber, b: ComplexNumber) -> ComplexNumber {
    let mut g: ComplexNumber;
    let mut g_prev: ComplexNumber;
    if a.is_greater_than(&b) {
        g = b;
        g_prev = a;
    } else {
        g = a;
        g_prev = b;
    }

    while !g.is_zero() {
        let tmp = g.clone();
        g = &g_prev - &(&g * &(&g_prev.div_round(&g)));
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
            real: BigInt::from(-6),
            imaginary: BigInt::from(17),
        };
        let y = ComplexNumber {
            real: BigInt::from(3),
            imaginary: BigInt::from(4),
        };

        assert_eq!(complex_euclidean_algorithm(y.clone(), x.clone()), y);
        assert_eq!(complex_euclidean_algorithm(x.clone(), y.clone()), y);
    }
}
