export interface GghKeyPairBean {
    dimension: number;
    good_basis: string[][];
    good_basis_inverse: string[][];
    unimodular_matrix: string[][];
    unimodular_matrix_inverse: string[][];
    bad_basis: string[][];
}

export interface GghEncryptRequest {
    message: string[];
    bad_basis: string[][];
    dimension: number;
    error_radius: number;
    random_seed: number;
}

export interface GghDecryptRequest {
    ciphertext: string[];
    good_basis: string[][];
    good_basis_inverse: string[][];
    unimodular_matrix_inverse: string[][];
    dimension: number;
}

export interface GghVectorBean {
    vector: string[];
}
