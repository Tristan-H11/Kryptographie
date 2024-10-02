/**
 * ShanksResponse model
 */
export class ShanksResponse {
    result: string;
    giantsteps: Array<[string, string]>;
    babysteps: Array<[string, string]>;

    constructor(result: string, giantsteps: [string, string][], babysteps: Array<[string, string]>) {
        this.result = result;
        this.giantsteps = giantsteps;
        this.babysteps = babysteps;
    }
}
