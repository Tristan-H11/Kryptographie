use nalgebra::{DMatrix, DVector};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// GGH mit ganzzahligen Matrizen (wie es sein sollte!)
pub type IntMatrix = DMatrix<i64>;
pub type IntVector = DVector<i64>;
pub type FloatMatrix = DMatrix<f64>;

/// GGH Schlüsselpaar
#[derive(Clone, Debug)]
pub struct GghKeyPair {
    pub private_key: GghPrivateKey,
    pub public_key: GghPublicKey,
}

/// Private (gute) Basis - annähernd orthogonal
#[derive(Clone, Debug)]
pub struct GghPrivateKey {
    /// Die gute (private) Basis (Spalten sind Basisvektoren)
    pub good_basis: IntMatrix,
    /// Inverse der guten Basis (für Entschlüsselung, als Float-Matrix)
    pub good_basis_inverse: FloatMatrix,
    /// Inverse der kumulierten unimodularen Matrix U (als Float-Matrix)
    pub unimodular_inverse: FloatMatrix,
    pub dimension: usize,
}


/// Public (schlechte) Basis
#[derive(Clone, Debug)]
pub struct GghPublicKey {
    /// Die schlechte (öffentliche) Basis (Spalten sind Basisvektoren)
    pub bad_basis: IntMatrix,
    pub dimension: usize,
}

/// Konfiguration für die GGH-Schlüsselgenerierung
#[derive(Clone, Debug)]
pub struct GghKeyGenConfig {
    pub dimension: usize,
    pub basis_vector_length: i64,
    pub unimodular_iterations: usize,
    pub random_seed: u64,
}

impl Default for GghKeyGenConfig {
    fn default() -> Self {
        Self {
            dimension: 4,
            basis_vector_length: 10,
            unimodular_iterations: 10,
            random_seed: 42,
        }
    }
}

/// GGH Verschlüsselungsschema
pub struct GghScheme;

impl GghScheme {
    /// Generiert ein GGH-Schlüsselpaar
    pub fn generate_keypair(config: &GghKeyGenConfig) -> GghKeyPair {
        // 1) gute Basis B und ihre Inverse
        let good_basis = Self::generate_good_basis(config);
        let good_basis_float = good_basis.map(|x| x as f64);
        let good_basis_inverse = good_basis_float
            .try_inverse()
            .expect("B sollte invertierbar sein (Diagonalmatrix).");

        // 2) schlechte Basis H = B * U und U selbst
        let (bad_basis, u_total) = Self::generate_bad_basis(&good_basis, config);

        // 3) U^{-1} als FloatMatrix (für einfache, robuste Demo-Rundung)
        let u_total_float = u_total.map(|x| x as f64);
        let unimodular_inverse = u_total_float
            .try_inverse()
            .expect("U muss invertierbar sein (unimodular).");

        GghKeyPair {
            private_key: GghPrivateKey {
                good_basis,
                good_basis_inverse,
                unimodular_inverse,
                dimension: config.dimension,
            },
            public_key: GghPublicKey {
                bad_basis,
                dimension: config.dimension,
            },
        }
    }


    /// Generiert eine gute Basis (Diagonalmatrix)
    fn generate_good_basis(config: &GghKeyGenConfig) -> IntMatrix {
        let mut basis = IntMatrix::zeros(config.dimension, config.dimension);
        for i in 0..config.dimension {
            basis[(i, i)] = config.basis_vector_length;
        }
        basis
    }

    /// Generiert eine schlechte Basis durch unimodulare Transformationen
    fn generate_bad_basis(good_basis: &IntMatrix, config: &GghKeyGenConfig) -> (IntMatrix, IntMatrix) {
        let mut result = good_basis.clone();                     // wird B * U
        let mut u_total = IntMatrix::identity(config.dimension, config.dimension); // U = I
        let mut rng = ChaCha8Rng::seed_from_u64(config.random_seed);

        for _ in 0..config.unimodular_iterations {
            let unimodular = Self::generate_unimodular_matrix(config.dimension, &mut rng);
            result = result * &unimodular;      // B := B * M
            u_total = u_total * unimodular;     // U := U * M
        }

        (result, u_total)
    }


    /// Generiert eine unimodulare Matrix (det = ±1)
    fn generate_unimodular_matrix(dimension: usize, rng: &mut ChaCha8Rng) -> IntMatrix {
        let mut matrix = IntMatrix::identity(dimension, dimension);

        for _ in 0..dimension * 2 {
            let i = rng.gen_range(0..dimension);
            let j = rng.gen_range(0..dimension);

            if i != j {
                let factor = rng.gen_range(-2..=2);
                // Spaltenoperation: Addiere factor * Spalte j zu Spalte i
                for k in 0..dimension {
                    matrix[(k, i)] += factor * matrix[(k, j)];
                }
            }
        }

        matrix
    }

    /// Verschlüsselt einen Vektor
    pub fn encrypt(
        message: &IntVector,
        public_key: &GghPublicKey,
        error_radius: i64,
        random_seed: u64,
    ) -> Result<IntVector, String> {
        if message.len() != public_key.dimension {
            return Err(format!(
                "Nachrichtendimension {} stimmt nicht mit Schlüsseldimension {} überein",
                message.len(),
                public_key.dimension
            ));
        }

        // Matrix-Vektor-Multiplikation mit nalgebra
        let mut encrypted = &public_key.bad_basis * message;

        // Füge ganzzahligen Fehlervektor hinzu
        let mut rng = ChaCha8Rng::seed_from_u64(random_seed);
        for i in 0..encrypted.len() {
            let error = rng.gen_range(-error_radius..=error_radius);
            encrypted[i] += error;
        }

        Ok(encrypted)
    }

    /// Entschlüsselt einen Vektor (Babai's Closest Vertex Algorithm)
    pub fn decrypt(ciphertext: &IntVector, private_key: &GghPrivateKey) -> Result<IntVector, String> {
        if ciphertext.len() != private_key.dimension {
            return Err(format!(
                "Ciphertext-Dimension {} stimmt nicht mit Schlüsseldimension {} überein",
                ciphertext.len(),
                private_key.dimension
            ));
        }

        // Schritt 1: x ≈ U m via Babai/Koordinatenrundung in der guten Basis
        let ciphertext_float = ciphertext.map(|x| x as f64);
        let coefficients = &private_key.good_basis_inverse * ciphertext_float; // B^{-1} c
        let rounded_coefficients_int = coefficients.map(|x| x.round() as i64); // x = round(B^{-1} c)

        // Schritt 2: m = U^{-1} x
        let x_float = rounded_coefficients_int.map(|t| t as f64);
        let m_float = &private_key.unimodular_inverse * x_float;
        let m_int = m_float.map(|v| v.round() as i64);

        Ok(m_int)
    }

    /// Berechnet die euklidische Distanz zwischen zwei Vektoren (nalgebra)
    pub fn vector_distance(a: &IntVector, b: &IntVector) -> f64 {
        // Konvertiere zu Float, da norm() ComplexField benötigt
        let diff = (a - b).map(|x| x as f64);
        diff.norm()
    }

    /// Gibt einen Vektor formatiert aus
    pub fn format_vector(v: &IntVector) -> String {
        format!("[{}]",
            v.iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    /// Gibt eine Matrix formatiert aus (Spalten sind Basisvektoren)
    pub fn format_matrix(m: &IntMatrix) -> String {
        let mut result = String::new();
        for i in 0..m.nrows() {
            result.push_str("[");
            for j in 0..m.ncols() {
                result.push_str(&format!("{:6}", m[(i, j)]));
                if j < m.ncols() - 1 {
                    result.push_str(", ");
                }
            }
            result.push_str("]\n");
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // SCHLÜSSELGENERIERUNG TESTS
    // ========================================================================

    #[test]
    fn test_keypair_generation_dimensions() {
        // Teste verschiedene Dimensionen
        for dimension in [2, 3, 4, 5, 8, 10] {
            let config = GghKeyGenConfig {
                dimension,
                basis_vector_length: 10,
                unimodular_iterations: 5,
                random_seed: 42,
            };
            let keypair = GghScheme::generate_keypair(&config);

            assert_eq!(keypair.private_key.dimension, dimension);
            assert_eq!(keypair.public_key.dimension, dimension);
            assert_eq!(keypair.private_key.good_basis.nrows(), dimension);
            assert_eq!(keypair.private_key.good_basis.ncols(), dimension);
            assert_eq!(keypair.public_key.bad_basis.nrows(), dimension);
            assert_eq!(keypair.public_key.bad_basis.ncols(), dimension);
        }
    }

    #[test]
    fn test_good_basis_is_diagonal() {
        let config = GghKeyGenConfig::default();
        let keypair = GghScheme::generate_keypair(&config);
        let basis = &keypair.private_key.good_basis;

        // Gute Basis muss Diagonalmatrix sein
        for i in 0..config.dimension {
            for j in 0..config.dimension {
                if i == j {
                    assert_eq!(
                        basis[(i, j)],
                        config.basis_vector_length,
                        "Diagonalelement ({}, {}) sollte basis_vector_length sein", i, j
                    );
                } else {
                    assert_eq!(
                        basis[(i, j)],
                        0,
                        "Nicht-Diagonalelement ({}, {}) sollte 0 sein", i, j
                    );
                }
            }
        }
    }

    #[test]
    fn test_good_basis_invertibility() {
        let config = GghKeyGenConfig::default();
        let keypair = GghScheme::generate_keypair(&config);

        // Prüfe dass Inverse existiert und B * B^(-1) ≈ I
        let basis_float = keypair.private_key.good_basis.map(|x| x as f64);
        let identity_approx = &basis_float * &keypair.private_key.good_basis_inverse;

        for i in 0..config.dimension {
            for j in 0..config.dimension {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (identity_approx[(i, j)] - expected).abs() < 1e-10,
                    "B * B^(-1) sollte Identität sein"
                );
            }
        }
    }

    #[test]
    fn test_unimodular_matrix_determinant() {
        // Teste mehrere unimodulare Matrizen
        for seed in [42, 123, 456, 789, 1000] {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let unimodular = GghScheme::generate_unimodular_matrix(4, &mut rng);
            let det = unimodular.map(|x| x as f64).determinant();

            assert!(
                (det.abs() - 1.0).abs() < 0.01,
                "Determinante sollte ±1 sein, ist aber {} (seed={})", det, seed
            );
        }
    }

    #[test]
    fn test_bad_basis_spans_same_lattice() {
        let config = GghKeyGenConfig {
            dimension: 3,
            basis_vector_length: 10,
            unimodular_iterations: 5,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);

        // Beide Basen sollten das gleiche Gitter aufspannen
        // Test: Jeder Vektor im Gitter der schlechten Basis sollte auch im Gitter der guten Basis sein
        let test_coeffs = IntVector::from_vec(vec![2, -3, 5]);
        let point_bad = &keypair.public_key.bad_basis * &test_coeffs;

        // Prüfe ob dieser Punkt durch die gute Basis darstellbar ist
        let point_float = point_bad.map(|x| x as f64);
        let coeffs_good = &keypair.private_key.good_basis_inverse * point_float;

        // Koeffizienten sollten ganzzahlig sein (bei exakter Darstellung)
        for coeff in coeffs_good.iter() {
            assert!(
                (coeff.round() - coeff).abs() < 1e-6,
                "Koeffizient {} sollte ganzzahlig sein für gleiches Gitter", coeff
            );
        }
    }

    #[test]
    fn test_reproducibility_with_same_seed() {
        let config = GghKeyGenConfig {
            dimension: 4,
            basis_vector_length: 10,
            unimodular_iterations: 5,
            random_seed: 42,
        };

        let keypair1 = GghScheme::generate_keypair(&config);
        let keypair2 = GghScheme::generate_keypair(&config);

        // Identischer Seed sollte identische Schlüssel erzeugen
        assert_eq!(keypair1.public_key.bad_basis, keypair2.public_key.bad_basis);
        assert_eq!(keypair1.private_key.good_basis, keypair2.private_key.good_basis);
    }

    // ========================================================================
    // VERSCHLÜSSELUNG/ENTSCHLÜSSELUNG TESTS
    // ========================================================================

    #[test]
    fn test_encrypt_decrypt_standard_messages() {
        let config = GghKeyGenConfig {
            dimension: 4,
            basis_vector_length: 20,
            unimodular_iterations: 5,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);
        let error_radius = 2;

        // Test verschiedene Standardnachrichten
        let test_messages = vec![
            vec![1, 2, 3, 4],
            vec![0, 0, 0, 0],
            vec![-1, 2, -3, 4],
            vec![10, -10, 5, -5],
            vec![1, 1, 1, 1],
        ];

        for (idx, msg_vec) in test_messages.iter().enumerate() {
            let message = IntVector::from_vec(msg_vec.clone());
            let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, error_radius, 123 + idx as u64)
                .expect("Verschlüsselung sollte funktionieren");
            let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key)
                .expect("Entschlüsselung sollte funktionieren");

            assert_eq!(
                decrypted, message,
                "Nachricht {:?} wurde nicht korrekt entschlüsselt", msg_vec
            );
        }
    }

    #[test]
    fn test_encrypt_decrypt_boundary_values() {
        let config = GghKeyGenConfig {
            dimension: 3,
            basis_vector_length: 50,
            unimodular_iterations: 3,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);

        // Test Grenzwerte - Verwende moderate große Zahlen, um Overflows zu vermeiden
        // Bei GGH können sehr große Zahlen zu Problemen führen, da:
        // - Ciphertext = Basis * Nachricht + Fehler
        // - Dies kann i64 Grenzen überschreiten
        let boundary_messages = vec![
            vec![100000, 0, 0],       // Große positive Zahl
            vec![-100000, 0, 0],      // Große negative Zahl
            vec![0, 0, 0],            // Null-Vektor
            vec![50000, -50000, 0],   // Gemischte große Werte
            vec![1000, 2000, 3000],   // Moderate Werte
        ];

        for msg_vec in boundary_messages {
            let message = IntVector::from_vec(msg_vec.clone());
            let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 1, 999)
                .expect("Verschlüsselung sollte funktionieren");
            let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key)
                .expect("Entschlüsselung sollte funktionieren");

            assert_eq!(decrypted, message, "Grenzwert-Nachricht {:?} fehlgeschlagen", msg_vec);
        }
    }

    #[test]
    fn test_overflow_prevention() {
        // Dieser Test dokumentiert die praktischen Grenzen von GGH
        // Bei sehr großen Nachrichten kann die Multiplikation Basis * Nachricht
        // zu i64 Overflow führen
        let config = GghKeyGenConfig {
            dimension: 3,
            basis_vector_length: 100,
            unimodular_iterations: 3,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);

        // Sichere Obergrenze: message[i] << basis_vector_length * max(bad_basis entries)
        // Für primitive GGH: max_safe_message ≈ i64::MAX / (basis_length * dimension * iterations)
        let safe_limit = 1_000_000; // Konservativ sicher

        let message = IntVector::from_vec(vec![safe_limit, -safe_limit, safe_limit / 2]);
        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 5, 123)
            .expect("Verschlüsselung sollte bei sicheren Werten funktionieren");
        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key)
            .expect("Entschlüsselung sollte funktionieren");

        assert_eq!(decrypted, message, "Verschlüsselung bei sicheren Grenzwerten fehlgeschlagen");
    }

    #[test]
    fn test_encrypt_dimension_mismatch() {
        let config = GghKeyGenConfig::default();
        let keypair = GghScheme::generate_keypair(&config);

        // Nachricht mit falscher Dimension
        let wrong_message = IntVector::from_vec(vec![1, 2, 3]); // Dimension 3 statt 4

        let result = GghScheme::encrypt(&wrong_message, &keypair.public_key, 2, 123);
        assert!(result.is_err(), "Verschlüsselung sollte bei falscher Dimension fehlschlagen");
        assert!(result.unwrap_err().contains("stimmt nicht"));
    }

    #[test]
    fn test_decrypt_dimension_mismatch() {
        let config = GghKeyGenConfig::default();
        let keypair = GghScheme::generate_keypair(&config);

        // Ciphertext mit falscher Dimension
        let wrong_ciphertext = IntVector::from_vec(vec![1, 2, 3]); // Dimension 3 statt 4

        let result = GghScheme::decrypt(&wrong_ciphertext, &keypair.private_key);
        assert!(result.is_err(), "Entschlüsselung sollte bei falscher Dimension fehlschlagen");
        assert!(result.unwrap_err().contains("stimmt nicht"));
    }

    #[test]
    fn test_error_radius_zero() {
        // Mit Fehlerradius 0 sollte c exakt auf dem Gitter liegen
        let config = GghKeyGenConfig {
            dimension: 3,
            basis_vector_length: 10,
            unimodular_iterations: 3,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);
        let message = IntVector::from_vec(vec![2, -1, 3]);

        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 0, 123).unwrap();
        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();

        assert_eq!(decrypted, message, "Bei Fehlerradius 0 sollte Entschlüsselung perfekt sein");
    }

    #[test]
    fn test_small_error_radius_success() {
        // Kleiner Fehlerradius sollte immer funktionieren
        let config = GghKeyGenConfig {
            dimension: 4,
            basis_vector_length: 50,
            unimodular_iterations: 5,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);

        // Fehlerradius deutlich kleiner als 0.5 * basis_vector_length
        let safe_error_radius = config.basis_vector_length / 10; // 10% der Basislänge

        for i in 0..10 {
            let message = IntVector::from_vec(vec![i, i+1, i+2, i+3]);
            let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, safe_error_radius, 100 + i as u64).unwrap();
            let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();

            assert_eq!(decrypted, message, "Iteration {} fehlgeschlagen", i);
        }
    }

    #[test]
    fn test_large_error_radius_failure() {
        // Zu großer Fehlerradius sollte Entschlüsselung fehlschlagen lassen
        let config = GghKeyGenConfig {
            dimension: 3,
            basis_vector_length: 10,
            unimodular_iterations: 3,
            random_seed: 43,
        };
        let keypair = GghScheme::generate_keypair(&config);
        let message = IntVector::from_vec(vec![1, 2, 3]);

        // Fehlerradius größer als 0.5 * basis_vector_length
        let unsafe_error_radius = (config.basis_vector_length as f64 * 0.8) as i64;

        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, unsafe_error_radius, 999).unwrap();
        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();

        // Bei zu großem Fehler ist die Entschlüsselung wahrscheinlich falsch
        let distance = GghScheme::vector_distance(&message, &decrypted);
        assert!(
            distance > 0.1 || decrypted != message,
            "Mit zu großem Fehlerradius sollte die Entschlüsselung oft fehlschlagen"
        );
    }

    #[test]
    fn test_critical_error_radius_boundary() {
        // Test genau an der Grenze: error_radius ≈ 0.5 * basis_vector_length
        let config = GghKeyGenConfig {
            dimension: 3,
            basis_vector_length: 20,
            unimodular_iterations: 3,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);
        let message = IntVector::from_vec(vec![5, -2, 7]);

        // Kritischer Fehlerradius (knapp unter der theoretischen Grenze)
        let critical_error_radius = (config.basis_vector_length as f64 * 0.45) as i64;

        let mut success_count = 0;
        let num_trials = 20;

        for trial in 0..num_trials {
            let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, critical_error_radius, 1000 + trial).unwrap();
            let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();

            if decrypted == message {
                success_count += 1;
            }
        }

        // Bei kritischem Fehlerradius sollten die meisten (aber nicht unbedingt alle) erfolgreich sein
        assert!(
            success_count > num_trials / 2,
            "Bei kritischem Fehlerradius sollten mindestens 50% erfolgreich sein, waren aber nur {}/{}",
            success_count, num_trials
        );
    }

    #[test]
    fn test_multiple_encryptions_same_message() {
        // Gleiche Nachricht sollte zu verschiedenen Ciphertexten führen (wegen Fehlervektor)
        let config = GghKeyGenConfig::default();
        let keypair = GghScheme::generate_keypair(&config);
        let message = IntVector::from_vec(vec![1, 2, 3, 4]);

        let ciphertext1 = GghScheme::encrypt(&message, &keypair.public_key, 2, 111).unwrap();
        let ciphertext2 = GghScheme::encrypt(&message, &keypair.public_key, 2, 222).unwrap();

        // Ciphertexte sollten unterschiedlich sein (probabilistische Verschlüsselung)
        assert_ne!(ciphertext1, ciphertext2, "Verschiedene Seeds sollten verschiedene Ciphertexte erzeugen");

        // Aber beide sollten zur gleichen Nachricht entschlüsseln
        let decrypted1 = GghScheme::decrypt(&ciphertext1, &keypair.private_key).unwrap();
        let decrypted2 = GghScheme::decrypt(&ciphertext2, &keypair.private_key).unwrap();

        assert_eq!(decrypted1, message);
        assert_eq!(decrypted2, message);
    }

    // ========================================================================
    // HILFSFUNKTIONEN TESTS
    // ========================================================================

    #[test]
    fn test_vector_distance_computation() {
        let a = IntVector::from_vec(vec![0, 0, 0]);
        let b = IntVector::from_vec(vec![3, 4, 0]);

        let distance = GghScheme::vector_distance(&a, &b);

        // 3-4-5 Dreieck
        assert!((distance - 5.0).abs() < 1e-10, "Distanz sollte 5.0 sein, ist aber {}", distance);
    }

    #[test]
    fn test_vector_distance_identity() {
        let a = IntVector::from_vec(vec![1, 2, 3, 4]);

        let distance = GghScheme::vector_distance(&a, &a);

        assert!(distance.abs() < 1e-10, "Distanz eines Vektors zu sich selbst sollte 0 sein");
    }

    #[test]
    fn test_format_vector_output() {
        let vec = IntVector::from_vec(vec![1, -5, 10]);
        let formatted = GghScheme::format_vector(&vec);

        assert!(formatted.contains("1"), "Formatierung sollte '1' enthalten");
        assert!(formatted.contains("-5"), "Formatierung sollte '-5' enthalten");
        assert!(formatted.contains("10"), "Formatierung sollte '10' enthalten");
        assert!(formatted.starts_with('['), "Formatierung sollte mit '[' beginnen");
        assert!(formatted.ends_with(']'), "Formatierung sollte mit ']' enden");
    }

    #[test]
    fn test_format_matrix_output() {
        let matrix = IntMatrix::from_vec(2, 2, vec![1, 2, 3, 4]);
        let formatted = GghScheme::format_matrix(&matrix);

        assert!(formatted.contains("1"), "Matrix-Formatierung sollte '1' enthalten");
        assert!(formatted.contains("4"), "Matrix-Formatierung sollte '4' enthalten");
        assert!(formatted.contains('\n'), "Matrix-Formatierung sollte Zeilenumbrüche haben");
    }

    // ========================================================================
    // PARAMETER-VARIATIONS TESTS
    // ========================================================================

    #[test]
    fn test_varying_basis_vector_lengths() {
        // Teste verschiedene Basislängen
        for length in [5, 10, 20, 50, 100] {
            let config = GghKeyGenConfig {
                dimension: 3,
                basis_vector_length: length,
                unimodular_iterations: 3,
                random_seed: 42,
            };
            let keypair = GghScheme::generate_keypair(&config);
            let message = IntVector::from_vec(vec![2, -1, 3]);

            // Angepasster Fehlerradius (10% der Basislänge)
            let error_radius = length / 10;

            let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, error_radius, 123).unwrap();
            let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();

            assert_eq!(decrypted, message, "Basislänge {} fehlgeschlagen", length);
        }
    }

    #[test]
    fn test_varying_unimodular_iterations() {
        // Teste verschiedene Anzahlen von unimodularen Transformationen
        for iterations in [1, 3, 5, 10, 20] {
            let config = GghKeyGenConfig {
                dimension: 3,
                basis_vector_length: 20,
                unimodular_iterations: iterations,
                random_seed: 42,
            };
            let keypair = GghScheme::generate_keypair(&config);
            let message = IntVector::from_vec(vec![1, 2, 3]);

            let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 2, 456).unwrap();
            let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();

            assert_eq!(decrypted, message, "Iterationen {} fehlgeschlagen", iterations);
        }
    }

    #[test]
    fn test_minimal_dimension() {
        // Minimale sinnvolle Dimension: 2
        let config = GghKeyGenConfig {
            dimension: 2,
            basis_vector_length: 10,
            unimodular_iterations: 3,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);
        let message = IntVector::from_vec(vec![3, -2]);

        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 1, 789).unwrap();
        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();

        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_large_dimension() {
        // Test mit größerer Dimension
        let dimension = 12;
        let config = GghKeyGenConfig {
            dimension,
            basis_vector_length: 15,
            unimodular_iterations: 5,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);
        let message = IntVector::from_vec((0..dimension as i64).collect());

        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 2, 999).unwrap();
        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();

        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_example_from_wikipedia() {
        // Exakte Nachstellung der Berechnung aus dem Foto:
        //
        // Gitter L ⊂ ℝ² mit Basis B und Inverse B^(-1):
        // B = (7  0)     B^(-1) = (1/7   0  )
        //     (0  3)               (0    1/3)
        //
        // Unimodulare Matrix U und Inverse U^(-1):
        // U = (2  3)     U^(-1) = ( 5  -3)
        //     (3  5)               (-3   2)
        //
        // Schlechte Basis: B' = BU = (14   9)
        //                             (21  15)
        //
        // Nachricht: m = (3, -7)
        // Fehlervektor: e = (1, -1)
        // Ciphertext: c = B'm + e = (-104, -79)
        //
        // Entschlüsselung:
        // 1. cB^(-1) = (-104/7, -79/3) ≈ (-14.857, -26.333)
        // 2. Rundung: (-15, -26)
        // 3. m = U^(-1)(-15, -26) = (3, -7) ✓

        // Manuelle Schlüsselerzeugung für exakte Kontrolle
        let dimension = 2;

        // Gute Basis B (Diagonalmatrix)
        // Verwende from_row_slice für intuitive Zeilen-Notation
        let good_basis = IntMatrix::from_row_slice(2, 2, &[
            7, 0,  // Zeile 0
            0, 3,  // Zeile 1
        ]);

        // B^(-1) als Float
        let good_basis_inverse = FloatMatrix::from_row_slice(2, 2, &[
            1.0/7.0, 0.0,
            0.0, 1.0/3.0,
        ]);

        // Unimodulare Matrix U
        let u_matrix = IntMatrix::from_row_slice(2, 2, &[
            2, 3,  // Zeile 0
            3, 5,  // Zeile 1
        ]);

        // U^(-1) als Float
        let u_inverse = FloatMatrix::from_row_slice(2, 2, &[
             5.0, -3.0,
            -3.0,  2.0,
        ]);

        // Schlechte Basis B' = UB (wie im Foto)
        let bad_basis = &u_matrix * &good_basis;

        // Verifiziere B' = (14   9)
        //                  (21  15)
        assert_eq!(bad_basis[(0, 0)], 14, "B'[0,0] sollte 14 sein");
        assert_eq!(bad_basis[(0, 1)], 9, "B'[0,1] sollte 9 sein");
        assert_eq!(bad_basis[(1, 0)], 21, "B'[1,0] sollte 21 sein");
        assert_eq!(bad_basis[(1, 1)], 15, "B'[1,1] sollte 15 sein");

        // Erstelle Schlüsselpaar manuell
        let private_key = GghPrivateKey {
            good_basis: good_basis.clone(),
            good_basis_inverse: good_basis_inverse.clone(),
            unimodular_inverse: u_inverse.clone(),
            dimension,
        };

        let public_key = GghPublicKey {
            bad_basis: bad_basis.clone(),
            dimension,
        };

        // Nachricht m = (3, -7)
        let message = IntVector::from_vec(vec![3, -7]);

        // Manuelle Verschlüsselung mit bekanntem Fehlervektor e = (1, -1)
        // Im Foto: c = mB' + e, wobei m ein Zeilenvektor ist
        // In nalgebra: c = B'^T * m (Transponierte!)
        let bad_basis_t = bad_basis.transpose();
        let mut ciphertext = &bad_basis_t * &message;
        // Füge Fehlervektor (1, -1) hinzu
        ciphertext[0] += 1;
        ciphertext[1] += -1;

        // Verifiziere c = (-104, -79)
        assert_eq!(ciphertext[0], -104, "Ciphertext[0] sollte -104 sein");
        assert_eq!(ciphertext[1], -79, "Ciphertext[1] sollte -79 sein");

        // Entschlüsselung
        let decrypted = GghScheme::decrypt(&ciphertext, &private_key)
            .expect("Entschlüsselung sollte funktionieren");

        // Verifiziere m = (3, -7)
        assert_eq!(decrypted[0], 3, "Entschlüsselte Nachricht[0] sollte 3 sein");
        assert_eq!(decrypted[1], -7, "Entschlüsselte Nachricht[1] sollte -7 sein");
        assert_eq!(decrypted, message, "Vollständige Nachricht sollte wiederhergestellt sein");

        // Zusätzliche Verifikationen der Zwischenschritte:

        // Schritt 1: cB^(-1) sollte ungefähr (-14.857, -26.333) sein
        let ciphertext_float = ciphertext.map(|x| x as f64);
        let coords = &good_basis_inverse * ciphertext_float;
        assert!((coords[0] - (-104.0/7.0)).abs() < 0.001, "coords[0] sollte -104/7 ≈ -14.857 sein");
        assert!((coords[1] - (-79.0/3.0)).abs() < 0.001, "coords[1] sollte -79/3 ≈ -26.333 sein");

        // Schritt 2: Rundung sollte (-15, -26) ergeben
        let rounded = coords.map(|x| x.round());
        assert_eq!(rounded[0], -15.0, "Gerundeter Wert[0] sollte -15 sein");
        assert_eq!(rounded[1], -26.0, "Gerundeter Wert[1] sollte -26 sein");

        // Schritt 3: (-15, -26)U^(-1) sollte (3, -7) ergeben
        let m_recovered = &u_inverse * rounded;
        assert!((m_recovered[0] - 3.0).abs() < 0.001, "Wiederhergestellte Nachricht[0] sollte 3 sein");
        assert!((m_recovered[1] - (-7.0)).abs() < 0.001, "Wiederhergestellte Nachricht[1] sollte -7 sein");
    }
}

