import {RsaKeyPair} from "./rsa-key-pair";

/**
 * Schnittstelle für die Anfrage zum Signieren einer Nachricht.
 */
export class RsaSignRequest {
	plaintext: string;
	key_pair: RsaKeyPair;
    radix: number;

	constructor(plaintext: string, keyPair: RsaKeyPair, radix: number) {
		this.plaintext = plaintext;
		this.key_pair = keyPair;
        this.radix = radix;
	}
}
