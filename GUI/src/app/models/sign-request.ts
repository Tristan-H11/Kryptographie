import {KeyPair} from "./key-pair";

export interface SignRequest {
    plaintext: string,
    key_pair: KeyPair
}

export function signRequestFrom(plaintext: string, keyPair: KeyPair): SignRequest {
    return {
        plaintext: plaintext,
        key_pair: keyPair
    }
}
