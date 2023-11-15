export interface MessageSignatureContainer {
  plaintext: string;
  ciphertext: string;
  signature: string;
}

export function getMessageSignatureFrom(plaintext: string, ciphertext: string, signature: string): MessageSignatureContainer {
  return {
    plaintext: plaintext,
    ciphertext: ciphertext,
    signature: signature
  }
}

export function createEmptyMessageSignatureContainer(): MessageSignatureContainer {
  return getMessageSignatureFrom("", "", "");
}
