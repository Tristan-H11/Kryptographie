use encryption_tool::encryption::core::ggh::ggh_integer::{GghScheme, GghKeyGenConfig, IntVector};
use num_bigint::ToBigInt;
use log::{info, debug, LevelFilter};
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_colors(true)
        .init()
        .unwrap();

    info!("=== GGH (Goldreich-Goldwasser-Halevi) Verschlüsselung ===");
    info!("");

    // ========================================================================
    // DEMO KONFIGURATION - Hier alle Parameter festlegen
    // ========================================================================

    // GGH Parameter
    const DIMENSION: usize = 3;
    const BASIS_VECTOR_LENGTH: i64 = 10;
    const UNIMODULAR_ITERATIONS: usize = 3;
    const RANDOM_SEED: u64 = 42;

    // Nachricht
    const RANDOM_MESSAGE: bool = false;
    const MESSAGE: [i64; DIMENSION] = [2, -1, 3];

    // Verschlüsselung
    const ERROR_RADIUS: i64 = 2;
    const ENCRYPTION_SEED: u64 = 123;

    // ========================================================================


    // Konfiguration
    let config = GghKeyGenConfig {
        dimension: DIMENSION,
        basis_vector_length: BASIS_VECTOR_LENGTH,
        unimodular_iterations: UNIMODULAR_ITERATIONS,
        random_seed: RANDOM_SEED,
    };

    info!("Konfiguration:");
    info!("  Dimension: {}", config.dimension);
    info!("  Basisvektor-Länge: {}", config.basis_vector_length);
    info!("  Unimodulare Iterationen: {}", config.unimodular_iterations);
    debug!("  Random Seed: {}", config.random_seed);
    info!("  Fehlerradius: {}", ERROR_RADIUS);
    debug!("  Zufällige Nachricht: {}", RANDOM_MESSAGE);
    info!("");

    // Schlüsselpaar generieren
    info!("=== SCHLÜSSELGENERIERUNG ===");
    let keypair = GghScheme::generate_keypair(&config);

    info!("");
    info!("Gute Basis (privat):");
    for line in GghScheme::format_matrix(&keypair.private_key.good_basis).lines() {
        info!("{}", line);
    }

    info!("");
    info!("Schlechte Basis (öffentlich):");
    for line in GghScheme::format_matrix(&keypair.public_key.bad_basis).lines() {
        info!("{}", line);
    }

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

    info!("");
    info!("=== VERSCHLÜSSELUNG ===");
    info!("Nachricht: {}", GghScheme::format_vector(&message));

    // Verschlüsseln
    let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, ERROR_RADIUS, ENCRYPTION_SEED)
        .expect("Verschlüsselung fehlgeschlagen");
    info!("Ciphertext: {}", GghScheme::format_vector(&ciphertext));

    // Entschlüsseln
    info!("");
    info!("=== ENTSCHLÜSSELUNG ===");
    let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key)
        .expect("Entschlüsselung fehlgeschlagen");
    info!("Entschlüsselte Nachricht: {}", GghScheme::format_vector(&decrypted));

    // Prüfen
    let green = "\x1b[32m";
    let red = "\x1b[31m";
    let reset = "\x1b[0m";

    info!("");
    if decrypted == message {
        info!("{}✓ Erfolgreich! Die entschlüsselte Nachricht stimmt mit dem Original überein.{}",
                 green, reset);
    } else {
        info!("{}✗ Fehler! Die entschlüsselte Nachricht stimmt NICHT mit dem Original überein.{}",
                 red, reset);
    }
}
