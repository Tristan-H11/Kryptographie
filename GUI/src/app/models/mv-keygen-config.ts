export class MvKeygenConfig { // TODO Mit MvCreateKeyPairRequest zusammenführen
    modulus_width: number;
    //todo Tristan, wieso wird hier nicht das Number System (Basis) mit angegeben?
    //in der menezesvanstone.component.ts wird in dem keygen jetzt nichtmehr das hier
    // verwendet sondern die menezes-vanestone-configuration-data.ts, damit es mit
    // dem RSA überein stimmt
    miller_rabin_rounds: number;
    coef_a: number;
    random_seed: number;

    constructor(modulus_width: number,
                miller_rabin_rounds: number,
                coef_a: number,
                random_seed: number) {
        this.modulus_width = modulus_width;
        this.miller_rabin_rounds = miller_rabin_rounds;
        this.coef_a = coef_a;
        this.random_seed = random_seed;
    }
}
