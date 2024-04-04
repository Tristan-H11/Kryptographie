export class MvKeyPair {
    publicKey: MvPublicKey;
    privateKey: MvPrivateKey;

    constructor(publicKey: MvPublicKey, privateKey: MvPrivateKey) {
        this.publicKey = publicKey;
        this.privateKey = privateKey;
    }
}

export class MvPublicKey {
    curve: EllipticCurve;
    generator: EllipticCurvePoint;
    y: EllipticCurvePoint;

    constructor(curve: EllipticCurve, generator: EllipticCurvePoint, y: EllipticCurvePoint) {
        this.curve = curve;
        this.generator = generator;
        this.y = y;
    }
}

export class MvPrivateKey {
    curve: EllipticCurve;
    x: string;

    constructor(curve: EllipticCurve, x: string) {
        this.curve = curve;
        this.x = x;
    }
}

export class EllipticCurve {
    a: number;
    b: number;
    p: string;

    constructor(a: number, b: number, p: string) {
        this.a = a;
        this.b = b;
        this.p = p;
    }
}

export class EllipticCurvePoint {
    x: string;
    y: string;

    constructor(x: string, y: string) {
        this.x = x;
        this.y = y;
    }
}
