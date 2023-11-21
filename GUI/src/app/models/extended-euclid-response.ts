export interface ExtendedEuclidResponse {
    ggt: string;
    x: string;
    y: string;
}

export function createExtendedEuclidResponseFrom(ggt: string, x: string, y: string): ExtendedEuclidResponse {
    return {
        ggt: ggt,
        x: x,
        y: y
    }
}
