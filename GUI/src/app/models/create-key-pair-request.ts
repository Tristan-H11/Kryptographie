export interface CreateKeyPairRequest {
  modulus_width: number;
  miller_rabin_rounds: number;
  random_seed: number;
  number_system_base: number;
}

/**
 * Erstellt ein CreateKeyPairRequest-Objekt.
 * @param modulus_width
 * @param miller_rabin_rounds
 * @param random_seed
 * @param number_system_base
 */
export function createKeyPairRequestFrom(modulus_width: number,
                                         miller_rabin_rounds: number,
                                         random_seed: number,
                                         number_system_base: number
): CreateKeyPairRequest {
  return {
    modulus_width,
    miller_rabin_rounds,
    random_seed,
    number_system_base
  };
}
