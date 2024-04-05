use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use num::integer::Average;
use std::ops::{Add, Div, Mul, Neg, Sub};

///Complex Number: a+bi
///mit entsprechenden Operationen
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexNumber {
    a: BigDecimal,
    b: BigDecimal,
}

impl Add for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: &self.a + &rhs.a,
            b: &self.b + &rhs.b,
        }
    }
}

impl Add for &ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            a: &self.a + &rhs.a,
            b: &self.b + &rhs.b,
        }
    }
}

impl Sub for ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a: &self.a - &rhs.a,
            b: &self.b - &rhs.b,
        }
    }
}

impl Sub for &ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            a: &self.a - &rhs.a,
            b: &self.b - &rhs.b,
        }
    }
}

impl Mul for ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            a: &self.a * &rhs.a - &self.b * &rhs.b,
            b: &self.a * &rhs.b + &self.b * &rhs.a,
        }
    }
}

impl Mul for &ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            a: &self.a * &rhs.a - &self.b * &rhs.b,
            b: &self.a * &rhs.b + &self.b * &rhs.a,
        }
    }
}

impl Div for ComplexNumber {
    type Output = ComplexNumber;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            a: (&self.a * &rhs.a + &self.b * &rhs.b) / (&rhs.a * &rhs.a + &rhs.b * &rhs.b),
            b: (&self.b * &rhs.a - &self.a * &rhs.b) / (&rhs.a * &rhs.a + &rhs.b * &rhs.b),
        }
    }
}

impl Div for &ComplexNumber {
    type Output = ComplexNumber;

    fn div(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            a: (&self.a * &rhs.a + &self.b * &rhs.b) / (&rhs.a * &rhs.a + &rhs.b * &rhs.b),
            b: (&self.b * &rhs.a - &self.a * &rhs.b) / (&rhs.a * &rhs.a + &rhs.b * &rhs.b),
        }
    }
}

impl ComplexNumber {
    pub fn complex_conjugate(self) -> Self {
        Self {
            a: self.a,
            b: self.b.neg(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.a.is_zero() && self.b.is_zero()
    }
    ///konvertiert beliebige komplexe Zahl in nächste Gauß'sche Zahl
    pub fn gaussian_integer(self) -> Self {
        Self {
            a: self.a.round(0),
            b: self.b.round(0),
        }
    }
}

///Berechnet ggT von zwei Gauß'schen Zahlen,
pub fn euclidean_algorithm(a: ComplexNumber, b: ComplexNumber) -> ComplexNumber {
    let mut g = a;
    let mut g_prev = b;

    while !g.is_zero() {
        let tmp = g.clone();
        g = &g_prev - &(&g * &((&g_prev / &g).gaussian_integer()));
        g_prev = tmp.clone();
    }
    ComplexNumber {
        a: g_prev.a.clone(),
        b: g_prev.b.clone(),
    }
}


#[cfg(test)]
mod complex_test {
    use bigdecimal::BigDecimal;
    use crate::encryption::core::el_gamal::complex_number::{ComplexNumber, euclidean_algorithm};

    #[test]
    fn complex_test() {
        let x = ComplexNumber{
            a: BigDecimal::from(-6),
            b: BigDecimal::from(17)
        };
        let y = ComplexNumber{
            a: BigDecimal::from(3),
            b: BigDecimal::from(4)
        };
        assert_eq!(euclidean_algorithm(y.clone(),x),y);
    }
}