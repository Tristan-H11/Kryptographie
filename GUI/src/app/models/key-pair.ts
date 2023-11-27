/**
 * Schnittstelle für ein KeyPair-Objekt.
 */
export class KeyPair {
	modulus: string;
	e: string;
	d: string;
	block_size_pub: string;
	block_size_priv: string;

	constructor(modulus: string, e: string, d: string, block_size_pub: string, block_size_priv: string) {
		this.modulus = modulus;
		this.e = e;
		this.d = d;
		this.block_size_pub = block_size_pub;
		this.block_size_priv = block_size_priv;
	}

	public static createEmptyKeyPair() {
		return new KeyPair("", "", "", "", "");
	}
}
