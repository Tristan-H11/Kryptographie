import {MvCipherText, MvKeyPair} from "./mv-beans";

/**
 * Darstellung der Clients.
 */
export class Client {
	public name = "";
	public sendingTo: Client | undefined;
	public receivedFrom: Client | undefined;

	constructor(name: string) {
		this.name = name;
	}
}

export interface ClientData {
	name: string;
	keyPair: MvKeyPair;
	plaintext: string;
	ciphertext: MvCipherText;
}