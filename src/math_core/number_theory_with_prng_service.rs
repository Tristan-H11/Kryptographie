use atomic_counter::{AtomicCounter, RelaxedCounter};
use bigdecimal::num_bigint::BigInt;

use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceSpeed, NumberTheoryServiceTrait,
};
use crate::math_core::pseudo_random_number_generator::PseudoRandomNumberGenerator;

/// Ein Wrapper für die Kombination aus NumberTheoryService und einem PseudoRandomNumberGenerator.
/// Dieser Wrapper ist stateful, weil er den Zähler des PRNG hält und verwaltet.
pub struct NumberTheoryWithPrngService {
    pub number_theory_service: NumberTheoryService,
    pub prng: PseudoRandomNumberGenerator,
    pub prng_counter: RelaxedCounter,
}

impl NumberTheoryWithPrngService {
    /// Erstellt eine neue Instanz des Wrappers mit der übergebenen Konfiguration.
    pub fn new(speed: NumberTheoryServiceSpeed, random_seed: u32) -> Self {
        let number_theory_service = NumberTheoryService::new(speed);
        let prng = PseudoRandomNumberGenerator::new(random_seed, number_theory_service);
        let prng_counter = RelaxedCounter::new(1);

        Self {
            number_theory_service,
            prng,
            prng_counter,
        }
    }

    /// Diese Methode gibt eine Zufallszahl im Bereich von a bis b zurück.
    /// Inkrementiert den AtomicCounter n_counter!
    ///
    /// # Argumente
    /// * `a` - Die untere Grenze des Bereichs.
    /// * `b` - Die obere Grenze des Bereichs.
    /// * `n_counter` - Der AtomicCounter, der den Index des Elementes aus der Zufallsfolge enthält.
    ///
    /// # Rückgabe
    /// Die Zufallszahl.
    pub fn take_random_number_in_range(&self, a: &BigInt, b: &BigInt) -> BigInt {
        // TODO Schnelle Lib Variante einbauen, je nach dem, welcher Speed hier gewrapped ist.
        self.prng.take(a, b, &self.prng_counter)
    }

    /// Diese Methode gibt eine ungerade Zufallszahl im Bereich von a bis b zurück.
    /// Inkrementiert den AtomicCounter n_counter!
    ///
    /// # Argumente
    /// * `a` - Die untere Grenze des Bereichs.
    /// * `b` - Die obere Grenze des Bereichs.
    /// * `n_counter` - Index des Elementes aus der Zufallsfolge.
    ///
    /// # Rückgabe
    /// Die ungerade Zufallszahl.
    pub fn take_random_uneven_number_in_range(&self, a: &BigInt, b: &BigInt) -> BigInt {
        // TODO Schnelle Lib Variante einbauen, je nach dem, welcher Speed hier gewrapped ist.
        self.prng.take_uneven(a, b, &self.prng_counter)
    }

    /// Generiert eine Primzahl mit der angegebenen Breite.
    ///
    /// # Argumente
    /// * `size` - Die Bit-Breite der Primzahl.
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    /// * `n_counter` - Der Zähler für den Zugriff auf die Zufallsfolge. Achtung: Der Zähler wird inkrementiert!
    ///
    /// # Rückgabe
    /// Die generierte Primzahl.
    pub fn generate_prime_with_width(&self, size: u32, miller_rabin_iterations: u32) -> BigInt {
        // TODO Schnelle Lib Variante einbauen, je nach dem, welcher Speed hier gewrapped ist.
        self.prng
            .generate_prime(size, miller_rabin_iterations, &self.prng_counter)
    }

    /// Prüft, ob die übergebene Zahl wahrscheinlich eine Primzahl ist.
    ///
    /// # Argumente
    /// * `p` - Die zu prüfende Zahl.
    /// * `repeats` - Die Anzahl der Wiederholungen für den Miller-Rabin-Test.
    ///
    /// # Rückgabe
    /// Wahr, wenn die Zahl wahrscheinlich eine Primzahl ist, sonst falsch.
    pub fn is_probably_prime(&self, p: &BigInt, repeats: u32) -> bool {
        self.number_theory_service
            .is_probably_prime(p, repeats, &self.prng)
    }

    /// Setzt den Wert des Counters für den PRNG zurück auf 1.
    pub fn reset_prng_counter(&mut self) {
        self.prng_counter.reset();
        self.prng_counter.inc();
    }
}
