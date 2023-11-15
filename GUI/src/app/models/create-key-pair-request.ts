export interface CreateKeyPairRequest {
  modulus_width: number;
  miller_rabin_rounds: number;
  random_seed: number;
  number_system_base: number;
}
