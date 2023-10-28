#[cfg(test)]
mod rsa_keys_test {
    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

    #[test]
    fn test_happy_flow_128() {
        // Intensiver Test, der die Verschlüsselung und Entschlüsselung wiederholt testet.
        let message = "bbbbbbbbbbbbbbb  äääääääääääääää";
        let keysize = 128;
        let keygen_service = RsaKeygenService::new(keysize);
        let mut counter = 0;

        for i in 0..15 {
            let (public_key, private_key) = keygen_service.generate_keypair(40);

            let encrypted_message = public_key.encrypt(message);
            println!("Verschlüsselte Nachricht: {}", encrypted_message);

            let decrypted_message = private_key.decrypt(&encrypted_message);
            if message != decrypted_message.trim() {
                println!("{} != {}", message, decrypted_message.trim());
                counter += 1;
            }
            // assert_eq!(message, decrypted_message.trim_end());
        }
        println!("{} Fehler", counter);
        assert_eq!(counter, 0)
    }
}
