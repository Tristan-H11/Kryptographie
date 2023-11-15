/**
 * Container für die Nachricht, den verschlüsselten Text und die Signatur.
 */
export interface MessageSignatureContainer {
  plaintext: string;
  ciphertext: string;
  signature: string;
}

/**
 * Erstellt ein MessageSignatureContainer-Objekt.
 */
export function getMessageSignatureFrom(plaintext: string, ciphertext: string, signature: string): MessageSignatureContainer {
  return {
    plaintext: plaintext,
    ciphertext: ciphertext,
    signature: signature
  }
}

/**
 * Erstellt ein leeres MessageSignatureContainer-Objekt.
 */
export function createEmptyMessageSignatureContainer(): MessageSignatureContainer {
  return getMessageSignatureFrom("", "", "");
}
