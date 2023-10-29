#[cfg(test)]
mod tests {
    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;
    use std::time::Instant;

    #[test]
    fn key_gen_timing_test() {
        // Erstellt Schlüsselpaare mit 2048 bit und berechnet die Durchschnittsdauer über n Läufe.
        let key_size = 1024;
        let keygen_service = RsaKeygenService::new(key_size);
        let mut times = Vec::new();
        let n = 5;

        for _i in 0..n {
            let start = Instant::now();
            keygen_service.generate_keypair(100);
            let end = Instant::now();
            times.push(end.duration_since(start).as_millis());
        }

        let sum: u128 = times.iter().sum();
        let avg = sum / n as u128;
        println!("Durchschnittliche Zeit: {} ms", avg);
    }
}
