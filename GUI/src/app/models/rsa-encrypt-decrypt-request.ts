import {RsaKeyPair} from "./rsa-key-pair";

/**
 * Schnittstelle für die Anfrage zum Verschlüsseln und Entschlüsseln einer Nachricht.
 */
export class RsaEncryptDecryptRequest {
	message: string;
	key_pair: RsaKeyPair;
	number_system_base: number;

	constructor(message: string, keyPair: RsaKeyPair, numberSystemBase: number) {
		this.message = message;
		this.key_pair = keyPair;
		this.number_system_base = numberSystemBase;
	}
}
