#[cfg(test)]
mod sign_verify_test {
    use std::str::FromStr;
    use crate::encryption::rsa::keys::{PrivateKey, PublicKey};
    use bigdecimal::num_bigint::BigInt;
    use crate::encryption::math_functions::number_theory::fast_exponentiation;
    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

    #[test]
    fn test_sign_and_verify_lowest_possible_happyflow() {
        let keygen = RsaKeygenService::new(257);

        let g_base = 55296;

        let (public_key, private_key) = keygen.generate_keypair(
            10,
            13,
            g_base
        );

        let message = "Die Nachricht soll signiert werden.";

        let signature = private_key.sign(&message);

        let is_valid = public_key.verify(&signature, &message);
        assert!(is_valid);
    }

    #[test]
    fn test_sign_and_verify_highest_unhappy_flow() {
        let keygen = RsaKeygenService::new(256);

        let g_base = 55296;

        let (public_key, private_key) = keygen.generate_keypair(
            10,
            13,
            g_base
        );

        let message = "Die Nachricht soll signiert werden.";

        let signature = private_key.sign(&message);

        let is_valid = public_key.verify(&signature, &message);
        // Assert NOT is_valid, because the key is too small and thus the message is truncated.
        assert!(!is_valid);
    }
}
