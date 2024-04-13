import {Injectable, signal, WritableSignal} from "@angular/core";
import {Client} from "../../encryption-components/shared/client";
import {RsaKeyPair} from "../../models/rsa-key-pair";
import {MessageSignatureContainer} from "../../models/message-signature-container";
import {RsaConfigurationData} from "../../models/rsa-configuration-data";

@Injectable({
    providedIn: "root"
})
export class StateManagementService {

    private server_url = signal("https://krypto-server.tristan-hoermann.de");

    private configurationDataRSA = signal(RsaConfigurationData.createDefaultConfigurationDataForRSA());

    private clientKeyMap = new Map<Client, WritableSignal<RsaKeyPair>>();

    private clientMessageMap = new Map<Client, WritableSignal<MessageSignatureContainer>>();

    private clients = new Set<Client>();

    private use_fast_math = signal(false);

    constructor() {
    }

    /**
     * Gibt die URL des Servers zurück.
     */
    public getServerUrl(): WritableSignal<string> {
        return this.server_url;
    }

    /**
     * Setzt die URL des Servers.
     */
    public setServerUrl(url: string): void {
        this.server_url.update(value => url);
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
        this.clientKeyMap.set(client, signal(RsaKeyPair.createEmptyKeyPair()));
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
        return this.configurationDataRSA;
    }

    public getClientKey(client: Client): WritableSignal<RsaKeyPair> {
        let entry = this.clientKeyMap.get(client);
        if (entry) {
            return entry;
        } else {
            console.log("Client " + client.name + " is not registered! Returning empty KeyPair and registering client.");
            this.clientKeyMap.set(client, signal(RsaKeyPair.createEmptyKeyPair()));
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
