/**
 * Modell für die Anfrage der Berechnung einer Exponentiation.
 */
export interface ExponentiationRequest {
  exponent: string;
  base: string;
  modulus: string;
}

export function createExponentiationRequestFrom(exponent: string, base: string, modulus: string): ExponentiationRequest {
  return {
    exponent: exponent,
    base: base,
    modulus: modulus
  }
}
