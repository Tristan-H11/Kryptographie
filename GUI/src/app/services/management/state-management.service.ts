import {Injectable, signal, WritableSignal} from "@angular/core";
import {Client} from "../../models/client";
import {KeyPair} from "../../models/key-pair";
import {MessageSignatureContainer} from "../../models/message-signature-container";
import {ConfigurationData} from "../../models/configuration-data";

@Injectable({
    providedIn: "root"
})
export class StateManagementService {

    private configurationData = signal(ConfigurationData.createDefaultConfigurationData());

    private clientKeyMap = new Map<Client, WritableSignal<KeyPair>>();

    private clientMessageMap = new Map<Client, WritableSignal<MessageSignatureContainer>>();

    private clients = new Set<Client>();

    private use_fast_math = signal(false);

    constructor() {
    }

    /**
     * Gibt das Signal für die Verwendung von FastMath zurück.
     */
    public getUseFastMath(): WritableSignal<boolean> {
        return this.use_fast_math;
    }

    /**
     * Erstellt einen Client und fügt ihn bei allen Services hinzu.
     */
    public createClient(clientName: string): void {
        let client = new Client(clientName);
        console.log("Registering client " + client.name + " at all services");
        this.clients.add(client);
        this.clientKeyMap.set(client, signal(KeyPair.createEmptyKeyPair()));
        this.clientMessageMap.set(client, signal({plaintext: "", ciphertext: "", signature: ""}));
    }

    /**
     * Gibt einen Client anhand seines Namens zurück.
     */
    public getClientByName(name: string): Client {
        for (let client of this.clients) {
            if (client.name === name) {
                return client;
            }
        }
        console.error("Client " + name + " not found! Creating empty client.");
        return new Client("");
    }

    /**
     * Gibt die Menge aller Clients zurück.
     */
    public getAllClients(): Set<Client> {
        return this.clients;
    }

    /**
     * Löscht einen Client und entfernt alle Registrierungen.
     */
    public deleteClient(client: Client): void {
        this.clients.delete(client);
        this.clientKeyMap.delete(client);
        this.clientMessageMap.delete(client);
    }

    /**
     * Gibt die Konfigurationsdaten zurück.
     */
    public getConfigurationData() {
        return this.configurationData;
    }

    public getClientKey(client: Client): WritableSignal<KeyPair> {
        let entry = this.clientKeyMap.get(client);
        if (entry) {
            return entry;
        } else {
            console.log("Client " + client.name + " is not registered! Returning empty KeyPair and registering client.");
            this.clientKeyMap.set(client, signal(KeyPair.createEmptyKeyPair()));
            return this.clientKeyMap.get(client)!; // Wir erstellen es ja in der Zeile davor
        }
    }

    /**
     * Gibt den MessageSignatureContainer für den Client zurück.
     */
    public getClientMessage(client: Client): WritableSignal<MessageSignatureContainer> {
        let entry = this.clientMessageMap.get(client);
        if (entry) {
            return entry;
        } else {
            console.log("Client " + client.name + " is not registered! Returning empty MessageSignatureContainer and registering client.");
            this.clientMessageMap.set(client, signal({plaintext: "", ciphertext: "", signature: ""}));
            return this.clientMessageMap.get(client)!; // Wir erstellen es ja in der Zeile davor
        }
    }

}
