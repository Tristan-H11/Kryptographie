import {KeyPair} from "./key-pair";

/**
 * Schnittstelle für die Anfrage zum Signieren einer Nachricht.
 */
export class SignRequest {
	plaintext: string;
	key_pair: KeyPair;
    g_base: number;

	constructor(plaintext: string, keyPair: KeyPair, g_base: number) {
		this.plaintext = plaintext;
		this.key_pair = keyPair;
        this.g_base = g_base;
	}
}
