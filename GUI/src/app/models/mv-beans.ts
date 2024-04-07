export interface EllipticCurve {
    a: number;
    prime: string;
    order_of_subgroup: string;
    generator: EcPoint;
}

export interface EcPoint {
    x: string;
    y: string;
    is_infinity: boolean;
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

export interface MvCreateKeyPairRequest {
    modulus_width: number;
    miller_rabin_rounds: number;
    coef_a: number;
    random_seed: number;
}

export interface MvEncryptRequest {
    public_key: MvPublicKey;
    message: string;
    radix: number;
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
