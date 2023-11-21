/**
 * Darstellung der Clients.
 */
export class Client {
  public name = "";
}

export function createNewClient(name: string): Client {
  return {name: name};
}
