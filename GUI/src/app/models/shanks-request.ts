/**
 * ShanksRequest model
 */
export class ShanksRequest {
    base: string;
    element: string;
    modul: string;

    constructor(base: string, element: string, modul: string) {
        this.base = base;
        this.element = element;
        this.modul = modul;
    }
}