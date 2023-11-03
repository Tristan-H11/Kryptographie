#[cfg(test)]
mod rsa_keys_test {
    use crate::big_i;
    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;
    use bigdecimal::num_bigint::BigInt;
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    #[test]
    fn test_happy_flow_1024() {
        // Intensiver Test, der die Verschlüsselung und Entschlüsselung wiederholt testet.
        let message = "bbbbbbbbbbbbbbb  äääääääääääääää  !&    ";
        let range = 20; // TODO hochstellen, wenn nötig

        let result = (0..range).into_par_iter().all(|_| {
            let keygen_service = RsaKeygenService::new(256);
            let (public_key, private_key) = keygen_service.generate_keypair(40, &big_i!(11));

            let encrypted_message = public_key.encrypt(message, 55296);
            println!("Verschlüsselte Nachricht: {}", encrypted_message);

            let decrypted_message = private_key.decrypt(&encrypted_message, 55296);
            message.trim_end() == decrypted_message
        });
        assert!(result);
    }
}
