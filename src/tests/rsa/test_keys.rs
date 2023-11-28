#[cfg(test)]
mod rsa_keys_test {
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

    #[test]
    fn test_happy_flow_1024() {
        // Intensiver Test, der die Verschlüsselung und Entschlüsselung wiederholt testet.
        let message = "bbbbbbbbbbbbbbb  äääääääääääääää  !&    ";
        let range = 2; // TODO hochstellen, wenn nötig

        let result = (0..range).into_par_iter().all(|_| {
            let keygen_service = RsaKeygenService::new(2048);
            let (public_key, private_key) = keygen_service.generate_keypair(40, 23, 55296, false); //TODO UseFast einbauen

            let encrypted_message = public_key.encrypt(message, 55296, false); //TODO UseFast einbauen
            println!("Verschlüsselte Nachricht: {}", encrypted_message);

            let decrypted_message = private_key.decrypt(&encrypted_message, 55296, false); //TODO UseFast einbauen
            message.trim_end() == decrypted_message
        });
        assert!(result);
    }

    #[test]
    fn test_happy_flow_1024_var_2() {
        let message = "Hallo wie geht es dir?";
        let keygen_service = RsaKeygenService::new(1024);
        let (public_key, private_key) = keygen_service.generate_keypair(40, 13, 55296, false); //TODO UseFast einbauen;

        let encrypted_message = public_key.encrypt(message, 55296, false); //TODO UseFast einbauen
        println!("Verschlüsselte Nachricht: {}", encrypted_message);

        let decrypted_message = private_key.decrypt(&encrypted_message, 55296, false); //TODO UseFast einbauen

        assert_eq!(message.trim_end(), decrypted_message);
    }
}
