use actix_web::web::Json;
use actix_web::{HttpResponse, Responder};
use log::info;
use num_bigint::BigInt;
use num_rational::BigRational;
use serde::{Deserialize, Serialize};

use crate::api::serializable_models::SingleStringResponse;
use crate::encryption::core::ggh::ggh_integer::{
    GghKeyGenConfig, GghPrivateKey, GghPublicKey, GghScheme, IntMatrix, IntVector, RationalMatrix,
};

// --- Request/Response Beans ---

#[derive(Deserialize)]
pub struct GghCreateKeyPairRequestBean {
    pub dimension: usize,
    pub basis_vector_length: i64,
    pub unimodular_iterations: usize,
    pub random_seed: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GghKeyPairBean {
    pub dimension: usize,
    pub good_basis: Vec<Vec<String>>,
    pub good_basis_inverse: Vec<Vec<String>>,
    pub unimodular_matrix: Vec<Vec<String>>,
    pub unimodular_matrix_inverse: Vec<Vec<String>>,
    pub bad_basis: Vec<Vec<String>>,
}

#[derive(Deserialize)]
pub struct GghEncryptRequestBean {
    pub message: Vec<String>,
    pub bad_basis: Vec<Vec<String>>,
    pub dimension: usize,
    pub error_radius: i64,
    pub random_seed: u64,
}

#[derive(Deserialize)]
pub struct GghDecryptRequestBean {
    pub ciphertext: Vec<String>,
    pub good_basis: Vec<Vec<String>>,
    pub good_basis_inverse: Vec<Vec<String>>,
    pub unimodular_matrix_inverse: Vec<Vec<String>>,
    pub dimension: usize,
}

#[derive(Serialize)]
pub struct GghVectorBean {
    pub vector: Vec<String>,
}

// --- Serialization helpers ---

fn matrix_to_bean(m: &IntMatrix) -> Vec<Vec<String>> {
    (0..m.nrows())
        .map(|i| (0..m.ncols()).map(|j| m[(i, j)].to_string()).collect())
        .collect()
}

fn rational_matrix_to_bean(m: &RationalMatrix) -> Vec<Vec<String>> {
    (0..m.nrows())
        .map(|i| (0..m.ncols()).map(|j| m[(i, j)].to_string()).collect())
        .collect()
}

fn vector_to_bean(v: &IntVector) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect()
}

fn bean_to_vector(v: &[String]) -> Result<IntVector, String> {
    let elements: Result<Vec<BigInt>, _> = v
        .iter()
        .map(|s| s.parse::<BigInt>().map_err(|e| e.to_string()))
        .collect();
    Ok(IntVector::from_vec(elements?))
}

fn bean_to_matrix(rows: &[Vec<String>]) -> Result<IntMatrix, String> {
    if rows.is_empty() {
        return Err("Leere Matrix".to_string());
    }
    let nrows = rows.len();
    let ncols = rows[0].len();
    let mut flat: Vec<BigInt> = Vec::with_capacity(nrows * ncols);
    for row in rows {
        for cell in row {
            flat.push(cell.parse::<BigInt>().map_err(|e| e.to_string())?);
        }
    }
    Ok(IntMatrix::from_row_slice(nrows, ncols, &flat))
}

fn bean_to_rational_matrix(rows: &[Vec<String>]) -> Result<RationalMatrix, String> {
    if rows.is_empty() {
        return Err("Leere Matrix".to_string());
    }
    let nrows = rows.len();
    let ncols = rows[0].len();
    let mut flat: Vec<BigRational> = Vec::with_capacity(nrows * ncols);
    for row in rows {
        for cell in row {
            flat.push(cell.parse::<BigRational>().map_err(|e| e.to_string())?);
        }
    }
    Ok(RationalMatrix::from_row_slice(nrows, ncols, &flat))
}

fn bad_request(message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(SingleStringResponse {
        message: message.to_string(),
    })
}

// --- Endpoint handlers ---

pub(crate) async fn create_key_pair(
    req_body: Json<GghCreateKeyPairRequestBean>,
) -> impl Responder {
    info!("Endpunkt /ggh/createKeyPair wurde aufgerufen");
    let req = req_body.into_inner();
    let config = GghKeyGenConfig {
        dimension: req.dimension,
        basis_vector_length: req.basis_vector_length,
        unimodular_iterations: req.unimodular_iterations,
        random_seed: req.random_seed,
    };

    let key_pair = GghScheme::generate_keypair(&config);
    let response = GghKeyPairBean {
        dimension: config.dimension,
        good_basis: matrix_to_bean(&key_pair.private_key.good_basis),
        good_basis_inverse: rational_matrix_to_bean(&key_pair.private_key.good_basis_inverse),
        unimodular_matrix: matrix_to_bean(&key_pair.private_key.unimodular_matrix),
        unimodular_matrix_inverse: matrix_to_bean(&key_pair.private_key.unimodular_matrix_inverse),
        bad_basis: matrix_to_bean(&key_pair.public_key.bad_basis),
    };
    HttpResponse::Ok().json(response)
}

pub(crate) async fn encrypt(req_body: Json<GghEncryptRequestBean>) -> impl Responder {
    info!("Endpunkt /ggh/encrypt wurde aufgerufen");
    let req = req_body.into_inner();

    let message = match bean_to_vector(&req.message) {
        Ok(v) => v,
        Err(e) => return bad_request(&format!("Fehler beim Parsen der Nachricht: {}", e)),
    };

    let bad_basis = match bean_to_matrix(&req.bad_basis) {
        Ok(m) => m,
        Err(e) => return bad_request(&format!("Fehler beim Parsen der Basis: {}", e)),
    };

    let public_key = GghPublicKey {
        bad_basis,
        dimension: req.dimension,
    };

    match GghScheme::encrypt(&message, &public_key, req.error_radius, req.random_seed) {
        Ok(ciphertext) => HttpResponse::Ok().json(GghVectorBean {
            vector: vector_to_bean(&ciphertext),
        }),
        Err(e) => bad_request(&format!("Verschlüsselung fehlgeschlagen: {}", e)),
    }
}

pub(crate) async fn decrypt(req_body: Json<GghDecryptRequestBean>) -> impl Responder {
    info!("Endpunkt /ggh/decrypt wurde aufgerufen");
    let req = req_body.into_inner();

    let ciphertext = match bean_to_vector(&req.ciphertext) {
        Ok(v) => v,
        Err(e) => return bad_request(&format!("Fehler beim Parsen des Geheimtextes: {}", e)),
    };

    let good_basis = match bean_to_matrix(&req.good_basis) {
        Ok(m) => m,
        Err(e) => return bad_request(&format!("Fehler beim Parsen der guten Basis: {}", e)),
    };

    let good_basis_inverse = match bean_to_rational_matrix(&req.good_basis_inverse) {
        Ok(m) => m,
        Err(e) => {
            return bad_request(&format!(
                "Fehler beim Parsen der inversen Basis: {}",
                e
            ))
        }
    };

    let unimodular_matrix_inverse = match bean_to_matrix(&req.unimodular_matrix_inverse) {
        Ok(m) => m,
        Err(e) => {
            return bad_request(&format!(
                "Fehler beim Parsen der unimodularen inversen Matrix: {}",
                e
            ))
        }
    };

    let private_key = GghPrivateKey {
        good_basis,
        good_basis_inverse,
        // unimodular_matrix is not needed for decryption
        unimodular_matrix: IntMatrix::identity(req.dimension, req.dimension),
        unimodular_matrix_inverse,
        dimension: req.dimension,
    };

    match GghScheme::decrypt(&ciphertext, &private_key) {
        Ok(message) => HttpResponse::Ok().json(GghVectorBean {
            vector: vector_to_bean(&message),
        }),
        Err(e) => bad_request(&format!("Entschlüsselung fehlgeschlagen: {}", e)),
    }
}
