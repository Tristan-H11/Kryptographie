import {KeyPair} from "./key-pair";

/**
 * Schnittstelle für die Anfrage zum Verifizieren einer Nachricht.
 */
export class VerifyRequest {
	plaintext: string;
	signature: string;
	key_pair: KeyPair;

	constructor(plaintext: string, signature: string, keyPair: KeyPair) {
		this.plaintext = plaintext;
		this.signature = signature;
		this.key_pair = keyPair;
	}
}
