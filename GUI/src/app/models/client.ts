/**
 * Darstellung der Clients.
 */
export class Client {
    public name = "";
    public sendingTo: Client | undefined;
    public receivedFrom: Client | undefined;

    constructor(name: string) {
        this.name = name;
    }
}
