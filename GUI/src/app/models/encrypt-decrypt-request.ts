import {KeyPair} from "./key-pair";

export interface EncryptDecryptRequest {
    message: string;
    key_pair: KeyPair;
    number_system_base: number;
}

export function createEncryptDecryptRequestFrom(message: string, keyPair: KeyPair, numberSystemBase: number): EncryptDecryptRequest {
    return {
        message: message,
        key_pair: keyPair,
        number_system_base: numberSystemBase
    }
}
