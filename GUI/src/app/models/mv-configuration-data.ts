/**
 * Class that represents the configuration data for the Menezes-Vanstone cryptosystem.
 */
export class MvConfigurationData {
    modulus_width: number;
    numberSystem: number;
    millerRabinIterations: number;
    coefficientA: number;
    random_seed: number;

    constructor(modulus_width: number,
                numberSystem: number,
                millerRabinIterations: number,
                coefficientA: number,
                random_seed: number
    ) {
        this.modulus_width = modulus_width;
        this.numberSystem = numberSystem;
        this.millerRabinIterations = millerRabinIterations;
        this.coefficientA = coefficientA;
        this.random_seed = random_seed;
    }

    public static createDefaultConfigurationDataForMV(): MvConfigurationData {
        return new MvConfigurationData(256, 55296, 10, 5, 3);
    }
}

