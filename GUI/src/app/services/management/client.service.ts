import {Injectable, OnInit} from '@angular/core';
import {Client, createNewClient} from "../../models/client";
import {AbstractClientObservableManagementService} from "./abstract-client-observable-management-service";
import {MessageManagementService} from "./message-management.service";
import {KeyManagementService} from "./key-management.service";

@Injectable({
    providedIn: 'root'
})
/**
 * Service zum Verwalten und Registrieren von Clients bei allen Client-Observable-Services.
 */
export class ClientService {

    private services = new Set<AbstractClientObservableManagementService<any>>()

    // Liste von Clients
    public clients = new Set<Client>();

    constructor(
        private messageService: MessageManagementService,
        private keyService: KeyManagementService,
    ) {}

    /**
     * Registriert alle Services, die Clients verwalten.
     */
    registerServices() {
        this.services.add(this.messageService);
        this.services.add(this.keyService);
    }

    /**
     * Registriert einen Client bei allen Services.
     */
    public registerClient(client: Client): void {
        console.log("Registering client " + client.name + " at all services")
        this.services.forEach(service => {
            console.log("Registering client " + client.name + " at " + service.constructor.name);
            service.registerClient(client)
        });
        console.log(this.clients)
    }

  /**
   * Löscht einen Client und entfernt alle Registrierungen.
   */
  public deleteAndUnregisterClient(client: Client): void {
    this.clients.delete(client);
    this.services.forEach(service => {
      console.log("Unregistering client " + client.name + " at " + service.constructor.name);
      service.unregisterClient(client)
    });
  }

  /**
     * Gibt die Liste aller Clients zurück.
     */
    public getClients(): Set<Client> {
        return this.clients;
    }

    /**
     * Erstellt einen neuen Client mit dem Namen und registriert ihn bei allen Services.
     */
    public createAndRegisterClient(name: string): Client {
        console.log("Creating new client " + name);
        let newClient: Client = createNewClient(name);
        this.clients.add(newClient);
        this.registerClient(newClient);
        return newClient;
    }

    /**
     * Gibt den Client mit dem Namen zurück.
     */
    public getClientByName(name: string): Client {
        for (let client of this.clients) {
            if (client.name === name) {
                return client;
            }
        }
        console.error("Client " + name + " not found!");
        return createNewClient("");
    }

}
