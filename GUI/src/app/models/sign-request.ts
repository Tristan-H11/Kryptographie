import {KeyPair} from "./key-pair";

/**
 * Schnittstelle für die Anfrage zum Signieren einer Nachricht.
 */
export class SignRequest {
	plaintext: string;
	key_pair: KeyPair;

	constructor(plaintext: string, keyPair: KeyPair) {
		this.plaintext = plaintext;
		this.key_pair = keyPair;
	}
}
