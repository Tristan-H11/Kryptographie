export interface ExtendedEuclidRequest {
    a: string;
    b: string;
}

export function createExtendedEuclidRequestFrom(a: string, b: string): ExtendedEuclidRequest {
    return {
        a: a,
        b: b
    }
}
