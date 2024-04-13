import {MvCipherText, MvKeyPair, MvSignature} from "../../models/mv-beans";

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

export interface IClientData {
    name: string;
    keyPair: any | undefined;
    plaintext: string;
    ciphertext: any | undefined;
    signature: any | undefined;
    signature_valid: string;
    sendingTo: IClientData | undefined;
    receivedFrom: IClientData | undefined;
}

export interface MvClientData extends IClientData{
	name: string;
	keyPair: MvKeyPair | undefined;
	plaintext: string;
	ciphertext: MvCipherText;
    signature: MvSignature;
    signature_valid: string;
    sendingTo: MvClientData | undefined;
    receivedFrom: MvClientData | undefined;
}

export function createDefaultMvClientData(name: string): MvClientData {
    return {
        receivedFrom: undefined, sendingTo: undefined,
        name: name,
        keyPair: {
            public_key: {
                curve: {
                    a: NaN, prime: "Empty",
                    generator: {
                        x: "Empty",
                        y: "Empty",
                        is_infinite: false
                    },
                    order_of_subgroup: "Empty"
                },
                y: {
                    x: "Empty",
                    y: "Empty",
                    is_infinite: false
                }
            },
            private_key: {
                curve: {
                    a: NaN, prime: "Empty",
                    generator: {
                        x: "Empty",
                        y: "Empty",
                        is_infinite: false
                    },
                    order_of_subgroup: "Empty"
                },
                x: "Empty"
            }
        },
        plaintext: "",
        ciphertext: {encrypted_message: "", points: []},
        signature: {r: "Empty", s: "Empty"},
        signature_valid: "ungeprüft"
    }
}
