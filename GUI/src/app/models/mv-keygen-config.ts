export class MvKeygenConfig {
    modulus_width: number;
    miller_rabin_rounds: number;
    coef_a: number;

    constructor(modulus_width: number,
                miller_rabin_rounds: number,
                coef_a: number) {
        this.modulus_width = modulus_width;
        this.miller_rabin_rounds = miller_rabin_rounds;
        this.coef_a = coef_a;
    }
}
