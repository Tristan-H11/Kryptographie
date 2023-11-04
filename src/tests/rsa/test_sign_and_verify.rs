#[cfg(test)]
mod sign_verify_test {
    use crate::encryption::rsa::keys::{PrivateKey, PublicKey};
    use bigdecimal::num_bigint::BigInt;

    #[test]
    fn test_sign_and_verify_flow() {
        let e = BigInt::from(65537);
        let n = BigInt::from(43);
        let d = BigInt::from(47);
        let g_base = BigInt::from(55296);

        let public_key = PublicKey::new(e.clone(), n.clone(), &g_base);
        let private_key = PrivateKey::new(d.clone(), n.clone(), &g_base);

        let message = "Hallo meine 123 ! kleine.";

        let signature = private_key.sign(&message);
        println!("Signatur: {}", signature);

        let is_valid = public_key.verify(&signature, &message);
        assert!(is_valid);
    }
}
