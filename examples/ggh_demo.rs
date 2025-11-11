use encryption_tool::encryption::core::ggh::ggh_integer::{GghScheme, GghKeyGenConfig, IntVector};
use num_bigint::ToBigInt;

fn main() {
    // ========================================================================
    // DEMO KONFIGURATION - Hier alle Parameter festlegen
    // ========================================================================

    // GGH Parameter
    const DIMENSION: usize = 4;
    const BASIS_VECTOR_LENGTH: i64 = 1000;
    const UNIMODULAR_ITERATIONS: usize = 1;
    const RANDOM_SEED: u64 = 42;

    // Nachricht
    const RANDOM_MESSAGE: bool = false;
    const MESSAGE: [i64; DIMENSION] = [3, -5, 7, 2];

    // Verschlüsselung
    const ERROR_RADIUS: i64 = 2;
    const ENCRYPTION_SEED: u64 = 123;

    // ========================================================================

    println!("=== GGH (Goldreich-Goldwasser-Halevi) Verschlüsselung ===\n");

    // Konfiguration
    let config = GghKeyGenConfig {
        dimension: DIMENSION,
        basis_vector_length: BASIS_VECTOR_LENGTH,
        unimodular_iterations: UNIMODULAR_ITERATIONS,
        random_seed: RANDOM_SEED,
    };

    println!("Konfiguration:");
    println!("  Dimension: {}", config.dimension);
    println!("  Basisvektor-Länge: {}", config.basis_vector_length);
    println!("  Unimodulare Iterationen: {}", config.unimodular_iterations);
    println!("  Random Seed: {}", config.random_seed);
    println!("  Fehlerradius: {}", ERROR_RADIUS);
    println!("  Zufällige Nachricht: {}", RANDOM_MESSAGE);
    println!();

    // Schlüsselpaar generieren
    println!("Generiere Schlüsselpaar...");
    let keypair = GghScheme::generate_keypair(&config);

    println!("\nGute Basis (privat):");
    println!("{}", GghScheme::format_matrix(&keypair.private_key.good_basis));

    println!("Schlechte Basis (öffentlich):");
    println!("{}", GghScheme::format_matrix(&keypair.public_key.bad_basis));

    // Nachricht
    let message = if RANDOM_MESSAGE {
        let mut vec = Vec::with_capacity(config.dimension);
        for _ in 0..config.dimension {
            let val = (rand::random::<i32>() % 21) - 10;
            vec.push(val.to_bigint().unwrap());
        }
        IntVector::from_vec(vec)
    } else {
        IntVector::from_vec(MESSAGE.iter().map(|&x| x.to_bigint().unwrap()).collect())
    };
    println!("Nachricht: {}", GghScheme::format_vector(&message));

    // Verschlüsseln
    let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, ERROR_RADIUS, ENCRYPTION_SEED)
        .expect("Verschlüsselung fehlgeschlagen");
    println!("Verschlüsselt (mit Fehlerradius {}): {}", ERROR_RADIUS, GghScheme::format_vector(&ciphertext));

    // Entschlüsseln
    let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key)
        .expect("Entschlüsselung fehlgeschlagen");
    println!("Entschlüsselt: {}", GghScheme::format_vector(&decrypted));

    // Prüfen
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";

    if decrypted == message {
        println!("\n{}✓ Erfolgreich! Die entschlüsselte Nachricht stimmt mit dem Original überein.{}",
                 green, reset);
    } else {
        println!("\n{}✗ Fehler! Die entschlüsselte Nachricht stimmt NICHT mit dem Original überein.{}",
                 red, reset);
    }
}
