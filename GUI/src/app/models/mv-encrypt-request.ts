import {EllipticCurvePoint, MvPrivateKey, MvPublicKey} from "./mv-key-pair";

export class MvEncryptRequest {
    publicKey: MvPublicKey;
    message: string;
    radix: number;

    constructor(publicKey: MvPublicKey, message: string, radix: number) {
        this.publicKey = publicKey;
        this.message = message;
        this.radix = radix;
    }
}

export class MvCipherText {
    encrypted_message: string;
    points: EllipticCurvePoint[];

    constructor(encrypted_message: string, points: EllipticCurvePoint[]) {
        this.encrypted_message = encrypted_message;
        this.points = points;
    }
}

export class MvDecryptRequest {
    privateKey: MvPrivateKey;
    cipherText: MvCipherText;
    radix: number;

    constructor(privateKey: MvPrivateKey, cipherText: MvCipherText, radix: number) {
        this.privateKey = privateKey;
        this.cipherText = cipherText;
        this.radix = radix;
    }
}
