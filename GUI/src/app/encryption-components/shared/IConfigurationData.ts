export interface IConfigurationData {
    modulusWidth: number;
    millerRabinRounds: number;
    randomSeed: number;
    numberSystem: number;
}

export interface MvConfigurationData extends IConfigurationData {
    modulusWidth: number;
    millerRabinRounds: number;
    randomSeed: number;
    numberSystem: number;
    coefA: number;
}

export interface RsaConfigurationData extends IConfigurationData {
}
