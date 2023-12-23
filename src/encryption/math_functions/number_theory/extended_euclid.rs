use num::{BigInt, Integer, One, Zero};

/// Implementiert den erweiterten euklidischen Algorithmus.
pub struct ExtendedEuclid {}

impl ExtendedEuclid {

    /// Berechnet den erweiterten euklidischen Algorithmus für zwei Zahlen.
    ///
    /// # Argumente
    ///
    /// * `a` - Die erste Zahl.
    /// * `b` - Die zweite Zahl.
    /// * `use_fast` - Wenn `true`, wird die schnelle Implementierung verwendet.
    ///
    /// # Rückgabewert
    ///
    /// * Ein Tripel aus dem größten gemeinsamen Teiler (`ggT`), dem Faktor `x` und dem Faktor `y`.
    ///
    /// # Beispiel
    ///
    /// ```rust
    /// let (ggT, x, y) = ExtendedEuclid::calculate(&BigInt::from(12), &BigInt::from(30));
    ///
    /// assert_eq!(ggT, BigInt::from(6));
    /// assert_eq!(x, BigInt::from(2));
    /// assert_eq!(y, BigInt::from(1));
    /// ```
    pub fn calculate(a: &BigInt, b: &BigInt, use_fast: bool) -> (BigInt, BigInt, BigInt) {
        return if use_fast {
            ExtendedEuclid::fast(a, b)
        } else {
            ExtendedEuclid::own(a, b)
        };
    }

    fn fast(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
        let e = a.extended_gcd(b);
        (e.gcd, e.x, e.y)
    }

    fn own(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
        //rotierendes Array, zur Berechnung und Speicherung der Faktoren `x` und `y`
        let mut xy = [BigInt::one(), BigInt::zero(), BigInt::zero(), BigInt::one()];
        let mut m = b.clone();
        let mut n = a.clone();
        while !m.is_zero() {
            // Berechnet die Faktoren und speichert sie in einem rotierenden Array.
            let div = &n / &m;
            xy[0] = &xy[0] - (&div * &xy[2]);
            xy[1] = &xy[1] - (&div * &xy[3]);
            let tmp = &n % &m;
            n = m;
            m = tmp;
            xy.rotate_right(2);
        }
        (n.clone(), xy[0].clone(), xy[1].clone())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::big_i;
    use super::*;
    #[test]
    fn extended_euclid_test() {
        assert_eq!(
            ExtendedEuclid::calculate(&big_i!(6), &big_i!(11), false),
            (big_i!(1), big_i!(2), big_i!(-1))
        );
        assert_eq!(
            ExtendedEuclid::calculate(&big_i!(78), &big_i!(99), false),
            (big_i!(3), big_i!(14), big_i!(-11))
        );
        assert_eq!(
            ExtendedEuclid::calculate(&big_i!(315), &big_i!(661643), false),
            (big_i!(1), big_i!(-319269), big_i!(152))
        );
        assert_eq!(
            ExtendedEuclid::calculate(&big_i!(315), &big_i!(661643), false),
            (big_i!(1), big_i!(-319269), big_i!(152))
        );
        assert_eq!(
            ExtendedEuclid::calculate(
                &BigInt::from_str("485398853520739824211578869461").unwrap(),
                &BigInt::from_str("79617341660363802320192939486040130094939703771377").unwrap(),
                false,
            ),
            (
                big_i!(1),
                BigInt::from_str("7173228757438794445922076835963679049602847038123").unwrap(),
                big_i!(-43732645957409398462249346726)
            )
        );
    }
}