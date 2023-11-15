export interface MessageSignature {
  plaintext: string;
  ciphertext: string;
  signature: string;
}

export function getMessageSignatureFrom(plaintext: string, ciphertext: string, signature: string): MessageSignature {
  return {
    plaintext: plaintext,
    ciphertext: ciphertext,
    signature: signature
  }
}

export function createEmptyMessageSignature(): MessageSignature {
  return getMessageSignatureFrom("", "", "");
}
