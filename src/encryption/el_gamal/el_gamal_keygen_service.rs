use crate::encryption::el_gamal::keys::{PrivateKey, PublicKey};
use crate::math_core::number_theory::number_theory_service::{
    NumberTheoryService, NumberTheoryServiceTrait,
};
use crate::math_core::pseudo_random_number_generator::PseudoRandomNumberGenerator;
use crate::math_core::traits::increment::Increment;
use atomic_counter::RelaxedCounter;
use log::debug;

///
/// Stellt einen Service zum Generieren von Schlüsselpaaren für das ElGamal-Kryptosystem in primen Restklassengruppen bereit.
///
pub struct ElGamalKeygenService {
    number_theory_service: NumberTheoryService,
    modulus_width: u32,
}

impl ElGamalKeygenService {
    ///
    /// Erstellt eine neue Instanz des ElGamalKeygenService.
    ///
    /// # Argumente
    /// * `number_theory_service` - Ein Service für die Durchführung von Operationen der Zahlentheorie.
    /// * `modulus_width` - Die Bitbreite des Modulus, unter dem die Restklassengruppe operiert.
    ///
    pub fn new(
        number_theory_service: NumberTheoryService,
        modulus_width: u32,
    ) -> ElGamalKeygenService {
        ElGamalKeygenService {
            number_theory_service,
            modulus_width,
        }
    }

    ///
    /// Generiert ein Schlüsselpaar für das ElGamal-Kryptosystem in primen Restklassengruppen.
    ///
    /// # Rückgabe
    /// Ein Tupel aus dem öffentlichen und privaten Schlüssel.
    ///
    pub(crate) fn generate_keypair(&self) -> (PublicKey, PrivateKey) {
        debug!("Generieren eines neuen ElGamal-Schlüsselpaares"); // TODO: Um Eingabeparameter ergänzen, wenn vorhanden.
        let random_generator = PseudoRandomNumberGenerator::new(13, self.number_theory_service); // TODO: Seed
        let counter = RelaxedCounter::new(1);

        // Generieren der sicheren Primzahl p und der Primitivwurzel g
        let (p, g) = random_generator.generate_secure_prime_with_primitive_root(
            self.modulus_width,
            100,
            &counter,
        ); //TODO: iterations

        // Generieren des privaten Schlüssels x (Zufallszahl zwischen 1 und p-2)
        let p_minus_two = p.decrement().decrement();
        let x = random_generator.take(&1.into(), &p_minus_two, &counter);

        // Berechnen des öffentlichen Schlüsselwertes y
        let y = self.number_theory_service.fast_exponentiation(&g, &x, &p);

        (PublicKey { p: p.clone(), g, y }, PrivateKey { p, x })
    }
}
