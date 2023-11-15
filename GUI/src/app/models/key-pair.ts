export interface KeyPair {
  modulus: string;
  e: string;
  d: string;
  block_size_pub: string;
  block_size_priv: string;
}

/**
 * Erstellt ein neues KeyPair-Objekt aus den übergebenen Parametern.
 * @param modulus Modulus
 * @param e Exponent
 * @param d Privater Exponent
 * @param block_size_pub Blockgröße für den öffentlichen Schlüssel
 * @param block_size_priv Blockgröße für den privaten Schlüssel
 */
export function createKeyPairFrom(modulus: string, e: string, d: string, block_size_pub: string, block_size_priv: string): KeyPair {
  return {
      modulus: modulus,
      e: e,
      block_size_pub: block_size_pub,
      d: d,
      block_size_priv: block_size_priv
    }
}

export function createEmptyKeyPair() {
  return createKeyPairFrom("", "", "", "", "");
}
