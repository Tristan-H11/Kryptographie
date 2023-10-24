use bigdecimal::num_bigint::{BigUint, ToBigInt};
use bigdecimal::One;
use log::{debug, trace};
use crate::big_u;
use crate::encryption::math_functions::big_int_util::{elsner_rand, is_one};
use crate::encryption::math_functions::number_theory::{extended_euclid, miller_rabin, modulo_inverse};
use crate::encryption::rsa::keys::{PublicKey, PrivateKey};

///
/// Ein Service zum Generieren von Schlüsselpaaren für RSA.
///
pub struct RsaKeygenService {
    key_size: usize,
}

impl RsaKeygenService {
    ///
    /// Erstellt eine neue Instanz des RsaKeygenService.
    ///
    /// # Argumente
    ///
    /// * `key_width` - Die Breite des Moduls `n`, mit welchem die Schlüssel berechnet werden.
    ///
    pub fn new(key_size: usize) -> RsaKeygenService {
        debug!("Creating new RsaKeygenService with key_size: {}", key_size);
        RsaKeygenService {
            key_size,
        }
    }

    ///
    /// Generiert ein Schlüsselpaar für RSA.
    ///
    /// # Argumente
    ///
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    ///
    /// # Rückgabe
    ///
    /// Ein Tupel aus dem öffentlichen und privaten Schlüssel.
    ///
    pub(crate) fn generate_keypair(&self, miller_rabin_iterations: usize) -> (PublicKey, PrivateKey) {
        debug!("Generating keypair with key_size: {}", self.key_size);
        let prim_size = self.key_size / 2;
        let prime_one = self.generate_prime(prim_size, miller_rabin_iterations);
        let mut prime_two = self.generate_prime(prim_size, miller_rabin_iterations);
        while prime_one == prime_two {
            prime_two = self.generate_prime(prim_size, miller_rabin_iterations);
        }

        let n = &prime_one * &prime_two;

        let phi = (&prime_one - BigUint::one()) * (&prime_two - BigUint::one());
        let e = self.generate_e(&phi);
        let d = self.generate_d(&e, &phi);
        let public_key = PublicKey::new(e, n.clone());
        let private_key = PrivateKey::new(d, n);
        debug!("Generated keypair");
        (public_key, private_key)
    }

    ///
    /// Generiert eine Primzahl mit der angegebenen Breite.
    ///
    /// # Argumente
    ///
    /// * `size` - Die Breite der Primzahl.
    /// * `miller_rabin_iterations` - Die Anzahl der Iterationen für den Miller-Rabin-Test.
    ///
    /// # Rückgabe
    ///
    /// Die generierte Primzahl.
    ///
    fn generate_prime(&self, size: usize, miller_rabin_iterations: usize) -> BigUint {
        debug!("Generating prime with size {} and Miller-Rabin-Iterations {}", size, miller_rabin_iterations);
        let upper_bound_prime_generation = &BigUint::from(2u8).pow(size as u32);
        let lower_bound_prime_generation = &BigUint::from(2u8).pow((size - 1) as u32);
        let random_int = rand::random::<u32>();
        let mut prime_candidate = BigUint::from(random_int); // TODO elsner_rand(lower_bound_prime_generation, upper_bound_prime_generation);

        //repeat random number until miller_rabin gives true
        while !miller_rabin(&prime_candidate, miller_rabin_iterations) {
            trace!("Generated prime candidate {} is not prime", prime_candidate);
            prime_candidate = BigUint::from(rand::random::<u32>()); // TODO elsner_rand(lower_bound_prime_generation, upper_bound_prime_generation);
        }
        debug!("Generated prime candidate {} is prime", prime_candidate);
        prime_candidate
    }

    ///
    /// Generiert eine Zahl `e` mit `1 < e < phi` und `ggT(e, phi) = 1`.
    ///
    /// # Argumente
    ///
    /// * `phi` - Die Zahl `phi`.
    ///
    /// # Rückgabe
    ///
    /// Die generierte Zahl `e`.
    ///
    fn generate_e(&self, phi: &BigUint) -> BigUint {
        debug!("Generating e with phi {}", phi);
        let mut e = big_u!(3u8); //TODO elsner_rand(&BigUint::from(3.0u8), &(phi - 1));
        while e < *phi {
            // Prüfen, ob e relativ prim zu phi ist, indem number_theory::extended_euclid() aufgerufen wird.
            //TODO Hübsch machen
            let euclid = &extended_euclid(&e.to_bigint().unwrap(), &phi.to_bigint().unwrap()).0.to_biguint().unwrap();
            if is_one(euclid)  {
                debug!("Generated e {}", e);
                return e;
            }
            trace!("Generated e {} is not relative prime to phi {}", e, phi);
            e += BigUint::one();
        }
        panic!("No e found");
    }

    ///
    /// Generiert eine Zahl `d` mit `1 < d < phi` und `e * d = 1 mod phi`.
    /// d ist damit das multiplikative Inverse von e mod phi.
    ///
    /// # Argumente
    ///
    /// * `e` - Die Zahl `e`.
    /// * `phi` - Die Zahl `phi`.
    ///
    /// # Rückgabe
    ///
    /// Die generierte Zahl `d`.
    ///
    fn generate_d(&self, e: &BigUint, phi: &BigUint) -> BigUint {
        debug!("Generating d with e {} and phi {}", e, phi);
        //TODO Hübsch machen
        modulo_inverse(e.to_bigint().unwrap(), phi.to_bigint().unwrap()).unwrap().to_biguint().unwrap()
    }
}
