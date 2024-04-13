export abstract class ConfigurationDataTypes {
    public modulusWidth: number;
    public millerRabinRounds: number;
    public randomSeed: number;
    public numberSystem: number;

    constructor(modulusWidth: number, millerRabinRounds: number, randomSeed: number, numberSystem: number) {
        this.modulusWidth = modulusWidth;
        this.millerRabinRounds = millerRabinRounds;
        this.randomSeed = randomSeed;
        this.numberSystem = numberSystem;
    }
}

export class MvConfigurationData extends ConfigurationDataTypes {
    public coefA: number;

    constructor(modulusWidth: number, millerRabinRounds: number, randomSeed: number, numberSystem: number, coefA: number) {
        super(modulusWidth, millerRabinRounds, randomSeed, numberSystem);
        this.modulusWidth = modulusWidth;
        this.millerRabinRounds = millerRabinRounds;
        this.randomSeed = randomSeed;
        this.numberSystem = numberSystem;
        this.coefA = coefA;
    }

    public static createDefault(): MvConfigurationData {
        return new MvConfigurationData(32, 40, 5, 55296, 3);
    }
}

export class RsaConfigurationData extends ConfigurationDataTypes {
    constructor(modulusWidth: number, millerRabinRounds: number, randomSeed: number, numberSystem: number) {
        super(modulusWidth, millerRabinRounds, randomSeed, numberSystem);
    }

    public static createDefault(): RsaConfigurationData {
        return new RsaConfigurationData(1024, 40, 3, 55296);
    }
}
