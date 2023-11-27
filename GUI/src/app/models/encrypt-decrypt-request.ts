import {KeyPair} from "./key-pair";

/**
 * Schnittstelle für die Anfrage zum Verschlüsseln und Entschlüsseln einer Nachricht.
 */
export class EncryptDecryptRequest {
	message: string;
	key_pair: KeyPair;
	number_system_base: number;

	constructor(message: string, keyPair: KeyPair, numberSystemBase: number) {
		this.message = message;
		this.key_pair = keyPair;
		this.number_system_base = numberSystemBase;
	}
}
