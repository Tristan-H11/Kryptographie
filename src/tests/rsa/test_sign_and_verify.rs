#[cfg(test)]
mod sign_verify_test {
    use bigdecimal::num_bigint::BigInt;
    use crate::big_i;
    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

    #[test]
    fn test_sign_and_verify_flow() {
        let random_seed = big_i!(17);
        let g_base = BigInt::from(55296);

        let key_gen = RsaKeygenService::new(1024);
        let (public_key, private_key) = key_gen.generate_keypair(40, &random_seed, &g_base);

        let message = "Hallo meine 123 ! kleine.";

        let signature = private_key.sign(&message);
        println!("Signatur: {}", signature);

        let is_valid = public_key.verify(&signature, &message);
        assert!(is_valid);
    }
}
