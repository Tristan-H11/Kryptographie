/**
 * ConfigurationData stellt die Datenstruktur für die Konfiguration der wichtigen RSA Eigenschaften bereit.
 */
export interface ConfigurationData {
  modulus_width: number;
  miller_rabin_rounds: number;
  random_seed: number;
  number_system_base: number;
}

/**
 * Erstellt ein ConfigurationData-Objekt.
 */
export function createConfigurationDataFrom(modulus_width: number,
                                            miller_rabin_rounds: number,
                                            random_seed: number,
                                            number_system_base: number
): ConfigurationData {
  return {
    modulus_width,
    miller_rabin_rounds,
    random_seed,
    number_system_base
  };
}

/**
 * Erstellt ein leeres ConfigurationData-Objekt.
 */
export function createDefaultConfigurationData(): ConfigurationData {
  return {
    modulus_width: 4096,
    miller_rabin_rounds: 100,
    random_seed: 13,
    number_system_base: 55296
  };
}