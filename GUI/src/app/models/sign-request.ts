import {KeyPair} from "./key-pair";

/**
 * Schnittstelle für die Anfrage zum Signieren einer Nachricht.
 */
export class SignRequest {
	plaintext: string;
	key_pair: KeyPair;
    radix: number;

	constructor(plaintext: string, keyPair: KeyPair, radix: number) {
		this.plaintext = plaintext;
		this.key_pair = keyPair;
        this.radix = radix;
	}
}
