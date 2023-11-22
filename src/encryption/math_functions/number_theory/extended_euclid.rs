use num::{BigInt, Integer, One, Zero};
use crate::encryption::math_functions::traits::rapid_math_ops::RapidMathOps;

/// Implementiert den erweiterten euklidischen Algorithmus.
///
/// Der erweiterte euklidische Algorithmus wird verwendet, um das Inverse-Element
/// einer Zahl in einem Restklassenring zu finden. Er arbeitet rekursiv und berechnet
/// die Faktoren `x` und `y` in der Bézout'schen Identität, so dass `x * n + y * modul = ggT(n, modul)`
///
/// # Argumente
/// * `n` - Die Zahl, welche mit dem Modul verechnet werden soll.
/// * `modul` - Die Modulo-Zahl, gegen die der Algorithmus durchgeführt wird.
///
/// # Rückgabe
/// * (ggT(n,modul),x,y)
/// Ein tripel aus dem groessten gemeinsamen Teiler einer Zahl `n` und dem `modul`,
/// sowie den zwei Faktoren `x` und `y`.
pub struct ExtendedEuclid {
    n: BigInt,
    modul: BigInt,
}

impl RapidMathOps<(BigInt, BigInt, BigInt)> for ExtendedEuclid {
    fn fast(&self) -> (BigInt, BigInt, BigInt) {
        let e = self.n.extended_gcd(&self.modul);
        (e.gcd, e.x, e.y)
    }

    fn own(&self) -> (BigInt, BigInt, BigInt) {
        self.extended_euclid(&self.n, &self.modul)
    }
}

impl ExtendedEuclid {
    /// Erstellt eine neue Instanz von ExtendedEuclid.
    ///
    /// # Argumente
    ///
    /// * `n` - Die Zahl, welche mit dem Modul verechnet werden soll.
    /// * `modul` - Die Modulo-Zahl, gegen die der Algorithmus durchgeführt wird.
    pub fn new(n: BigInt, modul: BigInt) -> Self {
        ExtendedEuclid {
            n,
            modul,
        }
    }

    /// Setzt das n neu, um kein neues Objekt erstellen zu müssen.
    pub fn setN(&mut self,

    /// Eigene Implementation des erweiterten Euklischen Algorithmus.
    fn extended_euclid(n: &BigInt, modul: &BigInt) -> (BigInt, BigInt, BigInt) {
        //rotierendes Array, zur Berechnung und Speicherung der Faktoren `x` und `y`
        let mut xy = [BigInt::one(), BigInt::zero(), BigInt::zero(), BigInt::one()];
        let mut m = modul.clone();
        let mut n = n.clone();
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
