/**
 * Container für die Nachricht, den verschlüsselten Text und die Signatur.
 */
export class MessageSignatureContainer {
	plaintext: string;
	ciphertext: string;
	signature: string;

	constructor(plaintext: string, ciphertext: string, signature: string) {
		this.plaintext = plaintext;
		this.ciphertext = ciphertext;
		this.signature = signature;
	}

	public static createEmptyMessageSignatureContainer() {
		return new MessageSignatureContainer("", "", "");
	}
}
