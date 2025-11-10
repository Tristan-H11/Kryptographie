use encryption_tool::encryption::core::ggh::ggh_integer::{GghScheme, GghKeyGenConfig, IntVector, GghKeyPair};

fn main() {
    println!("=== GGH (Goldreich-Goldwasser-Halevi) Verschlüsselung ===\n");

    // Konfiguration
    let config = GghKeyGenConfig {
        dimension: 7,
        basis_vector_length: 100000,
        unimodular_iterations: 10,
        random_seed: 42,
    };

    println!("Konfiguration:");
    println!("  Dimension: {}", config.dimension);
    println!("  Basisvektor-Länge: {}", config.basis_vector_length);
    println!("  Unimodulare Iterationen: {}", config.unimodular_iterations);
    println!();

    // Schlüsselpaar generieren
    println!("Generiere Schlüsselpaar...");
    let keypair = GghScheme::generate_keypair(&config);

    println!("\nGute Basis (privat):");
    println!("{}", GghScheme::format_matrix(&keypair.private_key.good_basis));

    println!("Schlechte Basis (öffentlich):");
    println!("{}", GghScheme::format_matrix(&keypair.public_key.bad_basis));

    // Nachricht
    let message = {
        let mut vec = Vec::with_capacity(config.dimension);
        for _ in 0..config.dimension as usize {
            // random values in range -10..=10
            let val = (rand::random::<i32>() % 21) - 10;
            vec.push(val as i64);
        }
        IntVector::from_vec(vec)
    };
    println!("Nachricht: {}", GghScheme::format_vector(&message));

    // Verschlüsseln
    let error_radius = 2;
    let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, error_radius, 123)
        .expect("Verschlüsselung fehlgeschlagen");
    println!("Verschlüsselt (mit Fehlerradius {}): {}", error_radius, GghScheme::format_vector(&ciphertext));

    // Entschlüsseln
    let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key)
        .expect("Entschlüsselung fehlgeschlagen");
    println!("Entschlüsselt: {}", GghScheme::format_vector(&decrypted));

    // Prüfen
    if decrypted == message {
        println!("\n✓ Erfolgreich! Die entschlüsselte Nachricht stimmt mit dem Original überein.");
    } else {
        println!("\n✗ Fehler! Die entschlüsselte Nachricht stimmt NICHT mit dem Original überein.");
    }
}

