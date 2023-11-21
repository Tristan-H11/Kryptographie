import {KeyPair} from "./key-pair";

/**
 * Schnittstelle für die Anfrage zum Verifizieren einer Nachricht.
 */
export interface VerifyRequest {
  plaintext: string,
  signature: string,
  key_pair: KeyPair
}

/**
 * Erstellt ein VerifyRequest-Objekt.
 */
export function verifyRequestFrom(plaintext: string, signature: string, keyPair: KeyPair): VerifyRequest {
  return {
    plaintext: plaintext,
    signature: signature,
    key_pair: keyPair
  }
}
