use nalgebra::{DMatrix, DVector};
use num_bigint::{BigInt, ToBigInt};
use num_rational::BigRational;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use num::{Zero, One, ToPrimitive};
use log::{debug, info, trace};

/// GGH mit ganzzahligen Matrizen (wie es sein sollte!)
pub type IntMatrix = DMatrix<BigInt>;
pub type IntVector = DVector<BigInt>;
pub type RationalMatrix = DMatrix<BigRational>;

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
    /// Inverse der guten Basis (für Entschlüsselung, als Rationale-Matrix)
    pub good_basis_inverse: RationalMatrix,
    /// Die unimodulare Matrix, die die gute in die schlechte Basis überführt.
    pub unimodular_matrix: IntMatrix,
    /// Die Inverse der unimodularen Matrix.
    pub unimodular_matrix_inverse: IntMatrix,
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
        info!("Starte GGH-Schlüsselgenerierung mit Dimension {}, Basisvektor-Länge {}, {} unimodulare Iterationen",
            config.dimension, config.basis_vector_length, config.unimodular_iterations);
        debug!("Konfiguration: seed={}", config.random_seed);

        // 1) gute Basis B und ihre Inverse
        trace!("Generiere gute (private) Basis als Diagonalmatrix");
        let good_basis = Self::generate_good_basis(config);
        debug!("Gute Basis generiert:\n{}", Self::format_matrix(&good_basis));

        trace!("Konvertiere gute Basis zu rationaler Matrix für Invertierung");
        let good_basis_rational = good_basis.map(|x| BigRational::from(x.clone()));

        trace!("Berechne Inverse der guten Basis");
        let good_basis_inverse = Self::try_inverse_rational(&good_basis_rational)
            .expect("B sollte invertierbar sein (Diagonalmatrix).");
        trace!("Inverse der guten Basis erfolgreich berechnet");

        // 2) schlechte Basis B' = B * U und die unimodulare Matrix U
        info!("Generiere schlechte (öffentliche) Basis durch unimodulare Transformationen");
        let (bad_basis, unimodular_matrix, unimodular_matrix_inverse) = Self::generate_bad_basis(&good_basis, config);
        debug!("Schlechte Basis generiert:\n{}", Self::format_matrix(&bad_basis));
        trace!("Unimodulare Matrix U_{}:\n{}", config.unimodular_iterations,  Self::format_matrix(&unimodular_matrix));

        info!("GGH-Schlüsselpaar erfolgreich generiert");

        GghKeyPair {
            private_key: GghPrivateKey {
                good_basis,
                good_basis_inverse,
                unimodular_matrix,
                unimodular_matrix_inverse,
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
        trace!("Erstelle Diagonalmatrix mit {} auf der Diagonale", config.basis_vector_length);
        let mut basis = IntMatrix::zeros(config.dimension, config.dimension);
        for i in 0..config.dimension {
            basis[(i, i)] = config.basis_vector_length.to_bigint().unwrap();
        }
        trace!("Gute Basis (Diagonalmatrix) erstellt");
        basis
    }

    /// Generiert eine schlechte Basis durch unimodulare Transformationen
    fn generate_bad_basis(good_basis: &IntMatrix, config: &GghKeyGenConfig) -> (IntMatrix, IntMatrix, IntMatrix) {
        debug!("Beginne mit Generierung der schlechten Basis ({} unimodulare Iterationen)", config.unimodular_iterations);
        let mut u_product = IntMatrix::identity(config.dimension, config.dimension);
        let mut rng = ChaCha8Rng::seed_from_u64(config.random_seed);

        for i in 0..config.unimodular_iterations {
            trace!("Unimodulare Iteration {}/{}", i + 1, config.unimodular_iterations);
            let unimodular = Self::generate_unimodular_matrix(config.dimension, &mut rng);
            u_product = u_product * &unimodular;
        }

        trace!("Berechne schlechte Basis: B' = B * U");
        let bad_basis = good_basis * &u_product;

        trace!("Invertiere unimodulare Matrix");
        let u_product_inverse = Self::invert_unimodular_int(&u_product)
            .expect("Das Produkt der unimodularen Matrizen sollte invertierbar sein.");

        debug!("Schlechte Basis erfolgreich generiert");

        (bad_basis, u_product, u_product_inverse)
    }


    /// Generiert eine unimodulare Matrix (det = ±1)
    fn generate_unimodular_matrix(dimension: usize, rng: &mut ChaCha8Rng) -> IntMatrix {
        trace!("Generiere unimodulare Matrix mit Dimension {}", dimension);
        let mut matrix = IntMatrix::identity(dimension, dimension);

        for op in 0..dimension * 2 {
            let i = rng.gen_range(0..dimension);
            let j = rng.gen_range(0..dimension);

            if i != j {
                let factor = rng.gen_range(-2..=2).to_bigint().unwrap();
                trace!("  Operation {}: Addiere {} * Spalte {} zu Spalte {}", op + 1, factor, j, i);
                // Spaltenoperation: Addiere factor * Spalte j zu Spalte i
                // Wir müssen den Wert von matrix[(k, j)] klonen, um den Borrow-Checker zufriedenzustellen,
                // da matrix gleichzeitig mutabel und immutabel ausgeliehen wird.
                for k in 0..dimension {
                    let val = matrix[(k, j)].clone();
                    matrix[(k, i)] += &factor * &val;
                }
            }
        }

        trace!("Unimodulare Matrix generiert");
        matrix
    }

    /// Invertiert eine ganzzahlige unimodulare Matrix exakt (det = ±1)
    fn invert_unimodular_int(matrix: &IntMatrix) -> Result<IntMatrix, String> {
        let n = matrix.nrows();
        trace!("Invertiere unimodulare {}x{} Matrix", n, n);

        // Konvertiere zu Rational für die Invertierung
        trace!("Konvertiere zu rationaler Matrix");
        let matrix_rational = matrix.map(|x| BigRational::from(x.clone()));
        let inverse_rational = Self::try_inverse_rational(&matrix_rational)
            .ok_or("Matrix ist nicht invertierbar")?;

        // Konvertiere zurück zu BigInt. Da die Matrix unimodular ist, muss die Inverse ganzzahlig sein.
        trace!("Konvertiere Inverse zurück zu ganzzahliger Matrix");
        let inverse_int = inverse_rational.map(|x| {
            if x.is_integer() {
                x.to_integer()
            } else {
                // Sollte nie passieren für eine unimodulare Matrix
                panic!("Inverse einer unimodularen Matrix ist nicht ganzzahlig.")
            }
        });

        // Verifiziere: M * M^(-1) = I
        trace!("Verifiziere: M * M^(-1) = I");
        let product = matrix * &inverse_int;
        let identity = IntMatrix::identity(n, n);

        if product != identity {
            return Err("Inverse ist nicht korrekt (Validierungsfehler)".to_string());
        }

        trace!("Inverse erfolgreich berechnet und verifiziert");
        Ok(inverse_int)
    }

    /// Verschlüsselt einen Vektor
    pub fn encrypt(
        message: &IntVector,
        public_key: &GghPublicKey,
        error_radius: i64,
        random_seed: u64,
    ) -> Result<IntVector, String> {
        info!("Starte GGH-Verschlüsselung mit Fehlerradius {}", error_radius);
        debug!("Nachricht: {}", Self::format_vector(message));

        if message.len() != public_key.dimension {
            return Err(format!(
                "Nachrichtendimension {} stimmt nicht mit Schlüsseldimension {} überein",
                message.len(),
                public_key.dimension
            ));
        }

        // Matrix-Vektor-Multiplikation mit nalgebra
        trace!("Berechne B' * m (Matrix-Vektor-Multiplikation)");
        let mut encrypted = &public_key.bad_basis * message;
        debug!("Nach Multiplikation mit schlechter Basis: {}", Self::format_vector(&encrypted));

        // Füge ganzzahligen Fehlervektor hinzu
        trace!("Generiere und addiere Fehlervektor (seed={})", random_seed);
        let mut rng = ChaCha8Rng::seed_from_u64(random_seed);
        let mut error_vec = Vec::with_capacity(encrypted.len());
        for i in 0..encrypted.len() {
            let error = rng.gen_range(-error_radius..=error_radius).to_bigint().unwrap();
            trace!("  Fehler[{}] = {}", i, error);
            error_vec.push(error.clone());
            encrypted[i] += error;
        }
        debug!("Fehlervektor: {}", Self::format_vector(&IntVector::from_vec(error_vec)));

        info!("Verschlüsselung erfolgreich abgeschlossen");
        debug!("Ciphertext: {}", Self::format_vector(&encrypted));

        Ok(encrypted)
    }

    /// Entschlüsselt einen Vektor (Babai's Closest Vertex Algorithm)
    pub fn decrypt(ciphertext: &IntVector, private_key: &GghPrivateKey) -> Result<IntVector, String> {
        info!("Starte GGH-Entschlüsselung mit Babai's Closest Vertex Algorithm");
        debug!("Ciphertext: {}", Self::format_vector(ciphertext));

        if ciphertext.len() != private_key.dimension {
            return Err(format!(
                "Ciphertext-Dimension {} stimmt nicht mit Schlüsseldimension {} überein",
                ciphertext.len(),
                private_key.dimension
            ));
        }

        // Schritt 1: Babai's Algorithm mit guter Basis
        // Gegeben: c = B * U₁ * U₂ * ... * Uₙ * m + e
        // Berechne: x = round(B⁻¹ * c) ≈ U * m
        trace!("Schritt 1: Konvertiere Ciphertext zu rationalen Zahlen");
        let ciphertext_rational = ciphertext.map(|x| BigRational::from(x.clone()));

        trace!("Schritt 2: Berechne B⁻¹ * c");
        let coefficients = &private_key.good_basis_inverse * ciphertext_rational;
        debug!("Koeffizienten vor Rundung: [{}]",
            coefficients.iter().map(|x| format!("{:.4}", x.to_f64().unwrap_or(0.0))).collect::<Vec<_>>().join(", "));

        trace!("Schritt 3: Runde Koeffizienten zu nächsten Ganzzahlen");
        let x_rounded = coefficients.map(|x| x.round().to_integer());
        debug!("Gerundete Koeffizienten x: {}", Self::format_vector(&x_rounded));

        // Schritt 2: Wende die inverse unimodulare Matrix an
        // m = U⁻¹ * x
        trace!("Schritt 4: Berechne m = U⁻¹ * x");
        let m = &private_key.unimodular_matrix_inverse * x_rounded;

        info!("Entschlüsselung erfolgreich abgeschlossen");
        debug!("Entschlüsselte Nachricht: {}", Self::format_vector(&m));

        Ok(m)
    }

    /// Berechnet die euklidische Distanz zwischen zwei Vektoren (nalgebra)
    pub fn vector_distance(a: &IntVector, b: &IntVector) -> f64 {
        // Konvertiere zu Float für die Norm-Berechnung
        let diff = (a - b).map(|x| {
            // Versuche, BigInt in f64 umzuwandeln.
            // Dies kann an Präzision verlieren für sehr große Zahlen, aber f��r die Distanz ist es oft ok.
            x.to_string().parse::<f64>().unwrap_or(f64::INFINITY)
        });
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

    // ========================================================================
    // Manuelle Implementierungen für BigRational-Matrizen
    // ========================================================================

    /// Berechnet die Determinante einer RationalMatrix mittels Gauß-Jordan-Elimination.
    fn determinant_rational(matrix: &RationalMatrix) -> BigRational {
        let mut mat = matrix.clone();
        let n = mat.nrows();
        if n != mat.ncols() {
            return BigRational::zero(); // Nur für quadratische Matrizen
        }

        let mut det = BigRational::one();

        for i in 0..n {
            // Finde Pivot
            let mut pivot_row = i;
            while pivot_row < n && mat[(pivot_row, i)].is_zero() {
                pivot_row += 1;
            }

            if pivot_row == n {
                return BigRational::zero(); // Keine eindeutige Lösung
            }

            if pivot_row != i {
                mat.swap_rows(i, pivot_row);
                det = -det;
            }

            let pivot_val = mat[(i, i)].clone();
            det *= &pivot_val;

            for j in i..n {
                mat[(i, j)] /= &pivot_val;
            }

            for row in 0..n {
                if row != i {
                    let factor = mat[(row, i)].clone();
                    for col in i..n {
                        let temp = mat[(i, col)].clone();
                        mat[(row, col)] -= &factor * &temp;
                    }
                }
            }
        }
        det
    }

    /// Invertiert eine RationalMatrix mittels Gauß-Jordan-Elimination.
    fn try_inverse_rational(matrix: &RationalMatrix) -> Option<RationalMatrix> {
        let n = matrix.nrows();
        if n != matrix.ncols() {
            return None; // Nur für quadratische Matrizen
        }

        let mut mat = matrix.clone();
        let mut inv = RationalMatrix::identity(n, n);

        for i in 0..n {
            // Finde Pivot
            let mut pivot_row = i;
            while pivot_row < n && mat[(pivot_row, i)].is_zero() {
                pivot_row += 1;
            }

            if pivot_row == n {
                return None; // Nicht invertierbar
            }

            mat.swap_rows(i, pivot_row);
            inv.swap_rows(i, pivot_row);

            let pivot_val = mat[(i, i)].clone();
            for j in 0..n {
                mat[(i, j)] /= &pivot_val;
                inv[(i, j)] /= &pivot_val;
            }

            for row in 0..n {
                if row != i {
                    let factor = mat[(row, i)].clone();
                    for col in 0..n {
                        let temp_mat = mat[(i, col)].clone();
                        let temp_inv = inv[(i, col)].clone();
                        mat[(row, col)] -= &factor * &temp_mat;
                        inv[(row, col)] -= &factor * &temp_inv;
                    }
                }
            }
        }
        Some(inv)
    }
}

#[cfg(test)]
mod tests {
    use num::{One, Signed, Zero};
    use super::*;
    use num_bigint::ToBigInt;

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
        let expected_val = config.basis_vector_length.to_bigint().unwrap();

        // Gute Basis muss Diagonalmatrix sein
        for i in 0..config.dimension {
            for j in 0..config.dimension {
                if i == j {
                    assert_eq!(
                        basis[(i, j)],
                        expected_val,
                        "Diagonalelement ({}, {}) sollte basis_vector_length sein", i, j
                    );
                } else {
                    assert_eq!(
                        basis[(i, j)],
                        BigInt::zero(),
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

        // Prüfe dass Inverse existiert und B * B^(-1) = I
        let basis_rational = keypair.private_key.good_basis.map(|x| BigRational::from(x.clone()));
        let identity_approx = &basis_rational * &keypair.private_key.good_basis_inverse;

        for i in 0..config.dimension {
            for j in 0..config.dimension {
                let expected = if i == j { BigRational::one() } else { BigRational::zero() };
                assert_eq!(identity_approx[(i, j)], expected, "B * B^(-1) sollte Identität sein");
            }
        }
    }

    #[test]
    fn test_unimodular_matrix_determinant() {
        // Teste mehrere unimodulare Matrizen
        for seed in [42, 123, 456, 789, 1000] {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let unimodular = GghScheme::generate_unimodular_matrix(4, &mut rng);
            let det = GghScheme::determinant_rational(&unimodular.map(|x| BigRational::from(x.clone())));

            assert_eq!(det.abs(), BigRational::one(), "Determinante sollte ±1 sein, ist aber {} (seed={})", det, seed);
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
        let test_coeffs = IntVector::from_vec(vec![2.to_bigint().unwrap(), (-3).to_bigint().unwrap(), 5.to_bigint().unwrap()]);
        let point_bad = &keypair.public_key.bad_basis * &test_coeffs;

        // Prüfe ob dieser Punkt durch die gute Basis darstellbar ist
        let point_rational = point_bad.map(|x| BigRational::from(x.clone()));
        let coeffs_good = &keypair.private_key.good_basis_inverse * point_rational;

        // Koeffizienten sollten ganzzahlig sein (bei exakter Darstellung)
        for coeff in coeffs_good.iter() {
            assert!(
                coeff.is_integer(),
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
            let message = IntVector::from_vec(msg_vec.iter().map(|&x| x.to_bigint().unwrap()).collect());
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
            let message = IntVector::from_vec(msg_vec.iter().map(|&x| x.to_bigint().unwrap()).collect());
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
        // zu i64 Overflow führen. Mit BigInt sollte dies kein Problem mehr sein.
        let config = GghKeyGenConfig {
            dimension: 3,
            basis_vector_length: 100,
            unimodular_iterations: 3,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);

        // Verwende eine Zahl, die i64 sprengen würde
        let large_val = BigInt::from(i64::MAX) * BigInt::from(100);

        let message = IntVector::from_vec(vec![large_val.clone(), -large_val.clone(), large_val / 2.to_bigint().unwrap()]);
        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 5, 123)
            .expect("Verschlüsselung sollte bei sicheren Werten funktionieren");
        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key)
            .expect("Entschlüsselung sollte funktionieren");

        assert_eq!(decrypted, message, "Verschlüsselung bei großen Werten fehlgeschlagen");
    }

    #[test]
    fn test_encrypt_dimension_mismatch() {
        let config = GghKeyGenConfig::default();
        let keypair = GghScheme::generate_keypair(&config);

        // Nachricht mit falscher Dimension
        let wrong_message = IntVector::from_vec(vec![1.to_bigint().unwrap(), 2.to_bigint().unwrap(), 3.to_bigint().unwrap()]); // Dimension 3 statt 4

        let result = GghScheme::encrypt(&wrong_message, &keypair.public_key, 2, 123);
        assert!(result.is_err(), "Verschlüsselung sollte bei falscher Dimension fehlschlagen");
        assert!(result.unwrap_err().contains("stimmt nicht"));
    }

    #[test]
    fn test_decrypt_dimension_mismatch() {
        let config = GghKeyGenConfig::default();
        let keypair = GghScheme::generate_keypair(&config);

        // Ciphertext mit falscher Dimension
        let wrong_ciphertext = IntVector::from_vec(vec![1.to_bigint().unwrap(), 2.to_bigint().unwrap(), 3.to_bigint().unwrap()]); // Dimension 3 statt 4

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
        let message = IntVector::from_vec(vec![2.to_bigint().unwrap(), (-1).to_bigint().unwrap(), 3.to_bigint().unwrap()]);

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
            let message = IntVector::from_vec(vec![i.to_bigint().unwrap(), (i+1).to_bigint().unwrap(), (i+2).to_bigint().unwrap(), (i+3).to_bigint().unwrap()]);
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
        let message = IntVector::from_vec(vec![1.to_bigint().unwrap(), 2.to_bigint().unwrap(), 3.to_bigint().unwrap()]);

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
        let message = IntVector::from_vec(vec![5.to_bigint().unwrap(), (-2).to_bigint().unwrap(), 7.to_bigint().unwrap()]);

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
        let message = IntVector::from_vec(vec![1.to_bigint().unwrap(), 2.to_bigint().unwrap(), 3.to_bigint().unwrap(), 4.to_bigint().unwrap()]);

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
        let a = IntVector::from_vec(vec![0.to_bigint().unwrap(), 0.to_bigint().unwrap(), 0.to_bigint().unwrap()]);
        let b = IntVector::from_vec(vec![3.to_bigint().unwrap(), 4.to_bigint().unwrap(), 0.to_bigint().unwrap()]);

        let distance = GghScheme::vector_distance(&a, &b);

        // 3-4-5 Dreieck
        assert!((distance - 5.0).abs() < 1e-10, "Distanz sollte 5.0 sein, ist aber {}", distance);
    }

    #[test]
    fn test_vector_distance_identity() {
        let a = IntVector::from_vec(vec![1.to_bigint().unwrap(), 2.to_bigint().unwrap(), 3.to_bigint().unwrap(), 4.to_bigint().unwrap()]);

        let distance = GghScheme::vector_distance(&a, &a);

        assert!(distance.abs() < 1e-10, "Distanz eines Vektors zu sich selbst sollte 0 sein");
    }

    #[test]
    fn test_format_vector_output() {
        let vec = IntVector::from_vec(vec![1.to_bigint().unwrap(), (-5).to_bigint().unwrap(), 10.to_bigint().unwrap()]);
        let formatted = GghScheme::format_vector(&vec);

        assert!(formatted.contains("1"), "Formatierung sollte '1' enthalten");
        assert!(formatted.contains("-5"), "Formatierung sollte '-5' enthalten");
        assert!(formatted.contains("10"), "Formatierung sollte '10' enthalten");
        assert!(formatted.starts_with('['), "Formatierung sollte mit '[' beginnen");
        assert!(formatted.ends_with(']'), "Formatierung sollte mit ']' enden");
    }

    #[test]
    fn test_format_matrix_output() {
        let matrix = IntMatrix::from_vec(2, 2, vec![1.to_bigint().unwrap(), 2.to_bigint().unwrap(), 3.to_bigint().unwrap(), 4.to_bigint().unwrap()]);
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
            let message = IntVector::from_vec(vec![2.to_bigint().unwrap(), (-1).to_bigint().unwrap(), 3.to_bigint().unwrap()]);

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
            let message = IntVector::from_vec(vec![1.to_bigint().unwrap(), 2.to_bigint().unwrap(), 3.to_bigint().unwrap()]);

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
        let message = IntVector::from_vec(vec![3.to_bigint().unwrap(), (-2).to_bigint().unwrap()]);

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
        let message = IntVector::from_vec((0..dimension as i64).map(|i| i.to_bigint().unwrap()).collect());

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
        // Schlechte Basis: B' = B * U = (14  21)  <- In Spalten-Major-Notation
        //                              ( 9  15)
        //
        // Nachricht: m = (3, -7)
        // Fehlervektor: e = (1, -1)
        // Ciphertext: c = B'm + e = (-104, -79)
        //
        // Entschlüsselung:
        // 1. B⁻¹c = (-104/7, -79/3) ≈ (-14.857, -26.333)
        // 2. Rundung: v = (-15, -26)
        // 3. m = U⁻¹v = (3, -7) ✓

        // Manuelle Schlüsselerzeugung für exakte Kontrolle
        let dimension = 2;

        // Gute Basis B (Diagonalmatrix)
        // Verwende from_row_slice für intuitive Zeilen-Notation
        let good_basis = IntMatrix::from_row_slice(2, 2, &[
            7.to_bigint().unwrap(), 0.to_bigint().unwrap(),  // Zeile 0
            0.to_bigint().unwrap(), 3.to_bigint().unwrap(),  // Zeile 1
        ]);

        // B^(-1) als Rational-Matrix
        let good_basis_inverse = RationalMatrix::from_row_slice(2, 2, &[
            BigRational::new(1.to_bigint().unwrap(), 7.to_bigint().unwrap()), BigRational::zero(),
            BigRational::zero(), BigRational::new(1.to_bigint().unwrap(), 3.to_bigint().unwrap()),
        ]);

        // Unimodulare Matrix U
        let u_matrix = IntMatrix::from_row_slice(2, 2, &[
            2.to_bigint().unwrap(), 3.to_bigint().unwrap(),  // Zeile 0
            3.to_bigint().unwrap(), 5.to_bigint().unwrap(),  // Zeile 1
        ]);

        // U^(-1) als Int-Matrix
        let u_inverse = IntMatrix::from_row_slice(2, 2, &[
             5.to_bigint().unwrap(), (-3).to_bigint().unwrap(),
            (-3).to_bigint().unwrap(),  2.to_bigint().unwrap(),
        ]);

        // Schlechte Basis B' = B * U (nalgebra ist Spalten-major, das ist die korrekte Reihenfolge)
        let bad_basis = &good_basis * &u_matrix;

        // Verifiziere B'
        // B * U = (7 0) * (2 3) = (14 21)
        //         (0 3)   (3 5)   ( 9 15)
        assert_eq!(bad_basis[(0, 0)], 14.to_bigint().unwrap());
        assert_eq!(bad_basis[(0, 1)], 21.to_bigint().unwrap());
        assert_eq!(bad_basis[(1, 0)], 9.to_bigint().unwrap());
        assert_eq!(bad_basis[(1, 1)], 15.to_bigint().unwrap());

        // Erstelle Schlüsselpaar manuell
        let private_key = GghPrivateKey {
            good_basis: good_basis.clone(),
            good_basis_inverse: good_basis_inverse.clone(),
            unimodular_matrix: u_matrix.clone(),
            unimodular_matrix_inverse: u_inverse.clone(),
            dimension,
        };

        let _public_key = GghPublicKey {
            bad_basis: bad_basis.clone(),
            dimension,
        };

        // Nachricht m = (3, -7)
        let message = IntVector::from_vec(vec![3.to_bigint().unwrap(), (-7).to_bigint().unwrap()]);

        // Manuelle Verschlüsselung mit bekanntem Fehlervektor e = (1, -1)
        // c = B'm + e
        let mut ciphertext = &bad_basis * &message;
        // Füge Fehlervektor (1, -1) hinzu
        ciphertext[0] += 1.to_bigint().unwrap();
        ciphertext[1] += (-1).to_bigint().unwrap();

        // Verifiziere c = (-104, -79)
        // B'm = (14*3 + 21*-7, 9*3 + 15*-7) = (42 - 147, 27 - 105) = (-105, -78)
        // c = (-105+1, -78-1) = (-104, -79)
        assert_eq!(ciphertext[0], (-104).to_bigint().unwrap(), "Ciphertext[0] sollte -104 sein");
        assert_eq!(ciphertext[1], (-79).to_bigint().unwrap(), "Ciphertext[1] sollte -79 sein");

        // Entschlüsselung
        let decrypted = GghScheme::decrypt(&ciphertext, &private_key)
            .expect("Entschlüsselung sollte funktionieren");

        // Verifiziere m = (3, -7)
        assert_eq!(decrypted[0], 3.to_bigint().unwrap(), "Entschlüsselte Nachricht[0] sollte 3 sein");
        assert_eq!(decrypted[1], (-7).to_bigint().unwrap(), "Entschlüsselte Nachricht[1] sollte -7 sein");
        assert_eq!(decrypted, message, "Vollständige Nachricht sollte wiederhergestellt sein");

        // Zusätzliche Verifikationen der Zwischenschritte:

        // Schritt 1: B⁻¹ * c sollte (-104/7, -79/3) sein
        let ciphertext_rational = ciphertext.map(|x| BigRational::from(x.clone()));
        let coords = &private_key.good_basis_inverse * ciphertext_rational;
        assert_eq!(coords[0], BigRational::new((-104).to_bigint().unwrap(), 7.to_bigint().unwrap()), "coords[0] sollte -104/7 sein");
        assert_eq!(coords[1], BigRational::new((-79).to_bigint().unwrap(), 3.to_bigint().unwrap()), "coords[1] sollte -79/3 sein");

        // Schritt 2: Rundung sollte (-15, -26) ergeben
        let rounded = coords.map(|x| x.round().to_integer());
        assert_eq!(rounded[0], (-15).to_bigint().unwrap(), "Gerundeter Wert[0] sollte -15 sein");
        assert_eq!(rounded[1], (-26).to_bigint().unwrap(), "Gerundeter Wert[1] sollte -26 sein");

        // Schritt 3: U⁻¹ * (-15, -26) sollte (3, -7) ergeben
        let rounded_int = rounded.map(|x| x.clone());
        let m_recovered_int = &u_inverse * rounded_int;
        assert_eq!(m_recovered_int[0], 3.to_bigint().unwrap(), "Wiederhergestellte Nachricht[0] sollte 3 sein");
        assert_eq!(m_recovered_int[1], (-7).to_bigint().unwrap(), "Wiederhergestellte Nachricht[1] sollte -7 sein");
    }
}

