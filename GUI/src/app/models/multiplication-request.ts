import {RsaKeyPair} from "./rsa-key-pair";

/**
 * Schnittstelle für die Anfrage zum RSA-Multiplizieren zweier Zahlen.
 */
export class MultiplicationRequest {
    factor_one: String;
    factor_two: String;
    key_pair: RsaKeyPair;

    constructor(factor_one: String, factor_two: String, key_pair: RsaKeyPair) {
        this.factor_one = factor_one;
        this.factor_two = factor_two;
        this.key_pair = key_pair;
    }
}
