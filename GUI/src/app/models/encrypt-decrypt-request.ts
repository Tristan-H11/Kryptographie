import {KeyPair} from "./key-pair";

/**
 * Schnittstelle für die Anfrage zum Verschlüsseln und Entschlüsseln einer Nachricht.
 */
export interface EncryptDecryptRequest {
  message: string;
  key_pair: KeyPair;
  number_system_base: number;
}

/**
 * Erstellt ein EncryptDecryptRequest-Objekt.
 */
export function createEncryptDecryptRequestFrom(message: string, keyPair: KeyPair, numberSystemBase: number): EncryptDecryptRequest {
  return {
    message: message,
    key_pair: keyPair,
    number_system_base: numberSystemBase
  }
}
