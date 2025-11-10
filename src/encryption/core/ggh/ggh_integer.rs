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

    #[test]
    fn test_keypair_generation() {
        let config = GghKeyGenConfig::default();
        let keypair = GghScheme::generate_keypair(&config);
        assert_eq!(keypair.private_key.dimension, config.dimension);
        assert_eq!(keypair.public_key.dimension, config.dimension);
    }

    #[test]
    fn test_encrypt_decrypt_simple() {
        let config = GghKeyGenConfig {
            dimension: 4,
            basis_vector_length: 10,
            unimodular_iterations: 5,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);

        println!("\n=== Test: Einfache Verschlüsselung ===");
        println!("Gute Basis (privat):");
        println!("{}", GghScheme::format_matrix(&keypair.private_key.good_basis));
        println!("Schlechte Basis (öffentlich):");
        println!("{}", GghScheme::format_matrix(&keypair.public_key.bad_basis));

        let message = IntVector::from_vec(vec![1, 2, 3, 4]);
        println!("Nachricht: {}", GghScheme::format_vector(&message));

        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 2, 123).unwrap();
        println!("Verschlüsselt: {}", GghScheme::format_vector(&ciphertext));

        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();
        println!("Entschlüsselt: {}", GghScheme::format_vector(&decrypted));

        assert_eq!(decrypted, message, "Entschlüsselung fehlgeschlagen!");
    }

    #[test]
    fn test_encrypt_decrypt_zero() {
        let config = GghKeyGenConfig::default();
        let keypair = GghScheme::generate_keypair(&config);
        let message = IntVector::from_vec(vec![0, 0, 0, 0]);
        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 2, 456).unwrap();
        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_encrypt_decrypt_negative() {
        let config = GghKeyGenConfig::default();
        let keypair = GghScheme::generate_keypair(&config);
        let message = IntVector::from_vec(vec![-1, 2, -3, 4]);
        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 2, 789).unwrap();
        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_unimodular_matrix() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let unimodular = GghScheme::generate_unimodular_matrix(3, &mut rng);
        let det = unimodular.map(|x| x as f64).determinant();
        println!("Determinante: {}", det);
        assert!((det.abs() - 1.0).abs() < 0.01, "Determinante sollte ±1 sein, ist aber {}", det);
    }

    #[test]
    fn test_different_dimensions() {
        let config = GghKeyGenConfig {
            dimension: 6,
            basis_vector_length: 15,
            unimodular_iterations: 8,
            random_seed: 42,
        };
        let keypair = GghScheme::generate_keypair(&config);
        let message = IntVector::from_vec(vec![1, 2, 3, 4, 5, 6]);
        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 3, 888).unwrap();
        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_manual_example() {
        let config = GghKeyGenConfig {
            dimension: 3,
            basis_vector_length: 5,
            unimodular_iterations: 3,
            random_seed: 12345,
        };
        let keypair = GghScheme::generate_keypair(&config);

        println!("\n=== Manuelles Beispiel ===");
        println!("Gute Basis (privat):");
        println!("{}", GghScheme::format_matrix(&keypair.private_key.good_basis));
        println!("Schlechte Basis (öffentlich):");
        println!("{}", GghScheme::format_matrix(&keypair.public_key.bad_basis));

        let message = IntVector::from_vec(vec![2, -1, 3]);
        println!("Nachricht: {}", GghScheme::format_vector(&message));

        let ciphertext = GghScheme::encrypt(&message, &keypair.public_key, 1, 99).unwrap();
        println!("Verschlüsselt: {}", GghScheme::format_vector(&ciphertext));

        let decrypted = GghScheme::decrypt(&ciphertext, &keypair.private_key).unwrap();
        println!("Entschlüsselt: {}", GghScheme::format_vector(&decrypted));

        assert_eq!(decrypted, message);
    }
}

