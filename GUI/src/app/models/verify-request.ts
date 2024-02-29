import {KeyPair} from "./key-pair";

/**
 * Schnittstelle für die Anfrage zum Verifizieren einer Nachricht.
 */
export class VerifyRequest {
	plaintext: string;
	signature: string;
	key_pair: KeyPair;
    radix: number;

	constructor(plaintext: string, signature: string, keyPair: KeyPair, radix: number) {
		this.plaintext = plaintext;
		this.signature = signature;
		this.key_pair = keyPair;
        this.radix = radix;
	}
}
