import {RsaKeyPair} from "./rsa-key-pair";

/**
 * Schnittstelle für die Anfrage zum Verifizieren einer Nachricht.
 */
export class RsaVerifyRequest {
	plaintext: string;
	signature: string;
	key_pair: RsaKeyPair;
    radix: number;

	constructor(plaintext: string, signature: string, keyPair: RsaKeyPair, radix: number) {
		this.plaintext = plaintext;
		this.signature = signature;
		this.key_pair = keyPair;
        this.radix = radix;
	}
}
