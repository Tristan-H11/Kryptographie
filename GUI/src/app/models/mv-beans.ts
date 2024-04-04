export interface EllipticCurve {
    a: number;
    b: number;
    prime: string;
}

export interface EcPoint {
    x: string;
    y: string;
}

export interface MvPublicKey {
    curve: EllipticCurve;
    generator: EcPoint;
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

// Deep Copy Function for EllipticCurve
export function copyEllipticCurve(curve: EllipticCurve): EllipticCurve {
    return { ...curve };
}

// Deep Copy Function for EcPoint
export function copyEcPoint(point: EcPoint): EcPoint {
    return { ...point };
}

// Deep Copy Function for MvPublicKey
export function copyMvPublicKey(publicKey: MvPublicKey): MvPublicKey {
    return {
        curve: copyEllipticCurve(publicKey.curve),
        generator: copyEcPoint(publicKey.generator),
        y: copyEcPoint(publicKey.y)
    };
}

// Deep Copy Function for MvPrivateKey
export function copyMvPrivateKey(privateKey: MvPrivateKey): MvPrivateKey {
    return {
        curve: copyEllipticCurve(privateKey.curve),
        x: privateKey.x
    };
}

// Deep Copy Function for MvBeans
export function copyMvKeyPair(keyPair: MvKeyPair): MvKeyPair {
    return {
        public_key: copyMvPublicKey(keyPair.public_key),
        private_key: copyMvPrivateKey(keyPair.private_key)
    };
}

// Deep Copy Function for MvEncryptRequest
export function copyMvEncryptRequest(request: MvEncryptRequest): MvEncryptRequest {
    return {
        public_key: copyMvPublicKey(request.public_key),
        message: request.message,
        radix: request.radix
    };
}

// Deep Copy Function for MvCipherText
export function copyMvCipherText(cipherText: MvCipherText): MvCipherText {
    return {
        encrypted_message: cipherText.encrypted_message,
        points: cipherText.points.map(copyEcPoint)
    };
}

// Deep Copy Function for MvDecryptRequest
export function copyMvDecryptRequest(request: MvDecryptRequest): MvDecryptRequest {
    return {
        private_key: copyMvPrivateKey(request.private_key),
        cipher_text: copyMvCipherText(request.cipher_text),
        radix: request.radix
    };
}
