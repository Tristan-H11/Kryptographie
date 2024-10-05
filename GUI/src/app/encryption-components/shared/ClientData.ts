import {MvCipherText, MvKeyPair, MvSignature} from "../../models/mv-beans";
import {RsaKeyPair} from "../../models/rsa-key-pair";

export abstract class ClientData {
    public name: string;
    public keyPair: MvKeyPair | RsaKeyPair | undefined;
    public plaintext: string;
    public ciphertext: string | MvCipherText;
    public signature: string | MvSignature | undefined;
    public signature_valid: string;
    public sendingTo: ClientData | undefined;
    public receivedFrom: ClientData | undefined;

    constructor(name: string, keyPair: any, plaintext: string, ciphertext: any, signature: any, signature_valid: string, sendingTo: ClientData | undefined, receivedFrom: ClientData | undefined) {
        this.name = name;
        this.keyPair = keyPair;
        this.plaintext = plaintext;
        this.ciphertext = ciphertext;
        this.signature = signature;
        this.signature_valid = signature_valid;
        this.sendingTo = sendingTo;
        this.receivedFrom = receivedFrom;
    }
}

export class RsaClientData extends ClientData {
    public override keyPair: RsaKeyPair | undefined;
    public override ciphertext: string;
    public override signature: string;
    public override sendingTo: RsaClientData | undefined;
    public override receivedFrom: RsaClientData | undefined;

    constructor(name: string, keyPair: RsaKeyPair | undefined, plaintext: string, ciphertext: string, signature: string, signature_valid: string, sendingTo: RsaClientData | undefined, receivedFrom: RsaClientData | undefined) {
        super(name, keyPair, plaintext, ciphertext, signature, signature_valid, sendingTo, receivedFrom);
        this.keyPair = keyPair;
        this.ciphertext = ciphertext;
        this.signature = signature;
        this.sendingTo = sendingTo;
        this.receivedFrom = receivedFrom;
    }

    public static createDefaultWithName(name: string): RsaClientData {
        return new RsaClientData(
            name,
            undefined,
            "",
            "",
            "",
            "ungeprüft",
            undefined,
            undefined);
    }
}

export class MvClientData extends ClientData {

    public override keyPair: MvKeyPair | undefined;
    public override ciphertext: MvCipherText;
    public override signature: MvSignature;
    public override sendingTo: MvClientData | undefined;
    public override receivedFrom: MvClientData | undefined;


    constructor(name: string, keyPair: any, plaintext: string, ciphertext: any, signature: any, signature_valid: string, sendingTo: ClientData | undefined, receivedFrom: ClientData | undefined) {
        super(name, keyPair, plaintext, ciphertext, signature, signature_valid, sendingTo, receivedFrom);
        this.keyPair = keyPair;
        this.ciphertext = ciphertext;
        this.signature = signature;
    }

    public static createDefaultWithName(name: string): MvClientData {
        return new MvClientData(
            name,
            undefined,
            "",
            {encrypted_message: "", points: []},
            {r: "", s: ""},
            "ungeprüft",
            undefined,
            undefined);

    }
}
