use std::ops::{Add, Div, Mul, Neg, Sub};
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use num::integer::Average;

///Complex Number: a+bi
#[derive(Debug, Clone)]
pub struct ComplexNumber {
    a:BigDecimal,
    b:BigDecimal
}

impl Add for ComplexNumber{
    type Output = ComplexNumber;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: &self.a + &rhs.a,
            b: &self.b + &rhs.b
        }
    }
}

impl Add for &ComplexNumber{
    type Output = ComplexNumber;

    fn add(self, rhs: Self) -> Self::Output {
        ComplexNumber{
            a: &self.a + &rhs.a,
            b: &self.b + &rhs.b
        }
    }
}

impl Sub for ComplexNumber{
    type Output = ComplexNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            a: &self.a - &rhs.a,
            b: &self.b - &rhs.b
        }
    }
}

impl Sub for &ComplexNumber{
    type Output = ComplexNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        ComplexNumber{
            a: &self.a - &rhs.a,
            b: &self.b - &rhs.b
        }
    }
}

impl Mul for ComplexNumber{
    type Output = ComplexNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            a: &self.a * &rhs.a - &self.b * &rhs.b,
            b: &self.a * &rhs.b + &self.b * &rhs.a
        }
    }
}

impl Mul for &ComplexNumber{
    type Output = ComplexNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        ComplexNumber{
            a: &self.a * &rhs.a - &self.b * &rhs.b,
            b: &self.a * &rhs.b + &self.b * &rhs.a
        }
    }
}

impl Div for ComplexNumber{
    type Output = ComplexNumber;

    fn div(self, rhs: Self) -> Self::Output {
        Self{
            a: (&self.a * &rhs.a + &self.b * &rhs.b) / (&rhs.a * &rhs.a + &rhs.b * &rhs.b),
            b: (&self.b * &rhs.a - &self.a * &rhs.b) / (&rhs.a * &rhs.a + &rhs.b * &rhs.b)
        }
    }
}

impl Div for &ComplexNumber{
    type Output = ComplexNumber;

    fn div(self, rhs: Self) -> Self::Output {
        ComplexNumber{
            a: (&self.a * &rhs.a + &self.b * &rhs.b) / (&rhs.a * &rhs.a + &rhs.b * &rhs.b),
            b: (&self.b * &rhs.a - &self.a * &rhs.b) / (&rhs.a * &rhs.a + &rhs.b * &rhs.b)
        }
    }
}
