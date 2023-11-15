import {KeyPair} from "./key-pair";

export interface VerifyRequest {
    plaintext: string,
    signature: string,
    key_pair: KeyPair
}

export function verifyRequestFrom(plaintext: string, signature: string, keyPair: KeyPair): VerifyRequest {
    return {
        plaintext: plaintext,
        signature: signature,
        key_pair: keyPair
    }
}
