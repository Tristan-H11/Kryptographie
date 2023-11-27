/**
 * Modell für die Anfrage der Berechnung einer Exponentiation.
 */
export class ExponentiationRequest {
	exponent: string;
	base: string;
	modulus: string;

	constructor(exponent: string, base: string, modulus: string) {
		this.exponent = exponent;
		this.base = base;
		this.modulus = modulus;
	}
}
