/**
 * Darstellung der Clients.
 */
export class Client {
    public name = "";
    public sendingTo: Client | undefined;
    public receivedFrom: Client | undefined;
}

export function createNewClient(name: string): Client {
    return {receivedFrom: undefined, sendingTo: undefined, name: name};
}
