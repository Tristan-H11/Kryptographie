#[cfg(test)]
mod sign_verify_test {
    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

    #[test]
    fn test_sign_and_verify_lowest_possible_happyflow() {
        let keygen = RsaKeygenService::new(258);

        let g_base = 55296;

        let (public_key, private_key) = keygen.generate_keypair(10, 17, g_base, false); //TODO UseFast einbauen

        let message = "Die Nachricht soll signiert werden.";

        let signature = private_key.sign(&message, false); //TODO UseFast einbauen

        let is_valid = public_key.verify(&signature, &message, false); //TODO UseFast einbauen
        assert!(is_valid);
    }

    #[test]
    fn test_sign_and_verify_highest_unhappy_flow() {
        let keygen = RsaKeygenService::new(256);

        let g_base = 55296;

        let (public_key, private_key) = keygen.generate_keypair(10, 13, g_base, false); //TODO UseFast einbauen

        let message = "Die Nachricht soll signiert werden.";

        let signature = private_key.sign(&message, false); //TODO UseFast einbauen

        let is_valid = public_key.verify(&signature, &message, false); //TODO UseFast einbauen
                                                                       // Assert NOT is_valid, because the key is too small and thus the message is truncated.
        assert!(!is_valid);
    }
}
