/**
 * Schnittstelle für die RSA-Multiplikationsantwort.
 */
export class MultiplicationResponse {
    encrypted_factor_one: String;
    encrypted_factor_two: String;
    encrypted_result: String;
    decrypted_result: String;

    constructor(encrypted_factor_one: String, encrypted_factor_two: String, encrypted_result: String, decrypted_result: String) {
        this.encrypted_factor_one = encrypted_factor_one;
        this.encrypted_factor_two = encrypted_factor_two;
        this.encrypted_result = encrypted_result;
        this.decrypted_result = decrypted_result;
    }
}
