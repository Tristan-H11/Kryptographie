use num::{BigInt, Integer, One, Zero};

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
pub struct ExtendedEuclid {}

impl ExtendedEuclid {
    /// Führt den erweiterten euklidischen Algorithmus aus.
    ///
    /// # Argumente
    ///
    /// * `n` - Die Zahl, welche mit dem Modul verechnet werden soll.
    /// * `modul` - Die Modulo-Zahl, gegen die der Algorithmus durchgeführt wird.
    /// * `use_fast` - Gibt an, ob die eigene Implementation oder die von `num` verwendet werden soll.
    pub fn calculate(n: &BigInt, modul: &BigInt, use_fast: bool) -> (BigInt, BigInt, BigInt) {
        return if use_fast {
            ExtendedEuclid::fast(n, modul)
        } else {
            ExtendedEuclid::own(n, modul)
        };
    }

    fn fast(n: &BigInt, modul: &BigInt) -> (BigInt, BigInt, BigInt) {
        let e = n.extended_gcd(modul);
        (e.gcd, e.x, e.y)
    }

    fn own(n: &BigInt, modul: &BigInt) -> (BigInt, BigInt, BigInt) {
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
