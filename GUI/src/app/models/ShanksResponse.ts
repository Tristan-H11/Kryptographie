/**
 * ShanksResponse model
 */
export class ShanksResponse {
    result: string;
    giantsteps: Array<[string, string]>;

    constructor(result: string, giantsteps: [string, string][]) {
        this.result = result;
        this.giantsteps = giantsteps;
    }
}
