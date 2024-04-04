/**
 * RsaConfigurationData stellt die Datenstruktur für die Konfiguration der wichtigen RSA Eigenschaften bereit.
 */
export class RsaConfigurationData {
	modulus_width: number;
	miller_rabin_rounds: number;
	random_seed: number;
	number_system_base: number;

	constructor(modulus_width: number,
				miller_rabin_rounds: number,
				random_seed: number,
				number_system_base: number) {
		this.modulus_width = modulus_width;
		this.miller_rabin_rounds = miller_rabin_rounds;
		this.random_seed = random_seed;
		this.number_system_base = number_system_base;
	}

	public static createDefaultConfigurationData(): RsaConfigurationData {
		return new RsaConfigurationData(4096, 100, 13, 55296);
	}
}

