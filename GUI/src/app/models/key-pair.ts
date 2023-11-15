import {PublicKey} from "./public-key";
import {PrivateKey} from "./private-key";

export interface KeyPair {
  public_key: PublicKey;
  private_key: PrivateKey;
}

/**
 * Erstellt ein neues KeyPair-Objekt aus den übergebenen Parametern.
 * @param modulus Modulus
 * @param e Exponent
 * @param d Privater Exponent
 * @param block_size_enc Blockgröße für die Verschlüsselung
 * @param block_size_dec Blockgröße für die Entschlüsselung
 */
export function createKeyPairFrom(modulus: string, e: string, d: string, block_size_enc: string, block_size_dec: string): KeyPair {
  return {
    public_key: {
      modulus: modulus,
      e: e,
      block_size: block_size_enc
    },
    private_key: {
      modulus: modulus,
      d: d,
      block_size: block_size_dec
    }
  }
}

export function createEmptyKeyPair() {
  return createKeyPairFrom("", "", "", "", "");
}
