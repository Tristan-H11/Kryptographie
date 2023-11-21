import {KeyPair} from "./key-pair";

/**
 * Schnittstelle für die Anfrage zum Signieren einer Nachricht.
 */
export interface SignRequest {
  plaintext: string,
  key_pair: KeyPair
}

/**
 * Erstellt ein SignRequest-Objekt.
 */
export function signRequestFrom(plaintext: string, keyPair: KeyPair): SignRequest {
  return {
    plaintext: plaintext,
    key_pair: keyPair
  }
}
