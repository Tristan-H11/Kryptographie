export interface EllipticCurve {
    a: number;
    prime: string;
    order_of_subgroup: string;
    generator: EcPoint;
}

export interface EcPoint {
    x: string;
    y: string;
    is_infinite: boolean;
}

export interface MvPublicKey {
    curve: EllipticCurve;
    y: EcPoint;
}

export interface MvPrivateKey {
    curve: EllipticCurve;
    x: string;
}

export interface MvKeyPair {
    public_key: MvPublicKey;
    private_key: MvPrivateKey;
}

export interface MvCreateKeyPairRequest { // TODO Mit Lucas KeyGenConfig zusammenführen
    modulus_width: number;
    miller_rabin_rounds: number;
    coef_a: number;
    random_seed: number;
}

export interface MvEncryptRequest {
    public_key: MvPublicKey;
    message: string;
    radix: number;
    random_seed: number,
}

export interface MvCipherText {
    encrypted_message: string;
    points: EcPoint[];
}

export interface MvDecryptRequest {
    private_key: MvPrivateKey;
    cipher_text: MvCipherText;
    radix: number;
}

export interface MvSignRequest {
    private_key: MvPrivateKey;
    message: string;
    random_seed: number;
}

export interface MvSignature {
    r: string,
    s: string,
    string_representation: string;
}

export interface MvVerifyRequest {
    public_key: MvPublicKey;
    message: string;
    signature: MvSignature;
}

export function copyEllipticCurve(curve: EllipticCurve): EllipticCurve {
    return {
        ...curve,
        generator: copyEcPoint(curve.generator)
    };
}

export function copyEcPoint(point: EcPoint): EcPoint {
    return { ...point };
}

export function copyMvPublicKey(publicKey: MvPublicKey): MvPublicKey {
    return {
        curve: copyEllipticCurve(publicKey.curve),
        y: copyEcPoint(publicKey.y)
    };
}

export function copyMvPrivateKey(privateKey: MvPrivateKey): MvPrivateKey {
    return {
        curve: copyEllipticCurve(privateKey.curve),
        x: privateKey.x
    };
}

export function copyMvKeyPair(keyPair: MvKeyPair): MvKeyPair {
    return {
        public_key: copyMvPublicKey(keyPair.public_key),
        private_key: copyMvPrivateKey(keyPair.private_key)
    };
}

export function copyMvCipherText(cipherText: MvCipherText): MvCipherText {
    return {
        encrypted_message: cipherText.encrypted_message,
        points: cipherText.points.map(copyEcPoint)
    };
}

export function copyMvSignRequest(signRequest: MvSignRequest): MvSignRequest {
    return {
        private_key: copyMvPrivateKey(signRequest.private_key),
        message: signRequest.message,
        random_seed: signRequest.random_seed
    };
}

export function copyMvSignature(signature: MvSignature): MvSignature {
    return {
        r: signature.r,
        s: signature.s,
        string_representation: signature.string_representation
    };
}

export function copyMvVerifyRequest(verifyRequest: MvVerifyRequest): MvVerifyRequest {
    return {
        public_key: copyMvPublicKey(verifyRequest.public_key),
        message: verifyRequest.message,
        signature: copyMvSignature(verifyRequest.signature)
    };
}
