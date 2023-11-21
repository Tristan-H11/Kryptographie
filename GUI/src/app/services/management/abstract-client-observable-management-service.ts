import {Client} from "../../models/client";
import {BehaviorSubject, Observable} from "rxjs";

/**
 * Abstrakte Klasse, die die Verwaltung von Clients mit BehaviorSubjects übernimmt.
 */
export abstract class AbstractClientObservableManagementService<T> {

    /**
     * Map mit allen Clients und den dazugehörigen BehaviorSubjects.
     */
    protected clientMap = new Map<Client, BehaviorSubject<T>>();

    protected constructor() {
    }

    /**
     * Erstellt ein default (meist leeres) Objekt vom Typ T.
     */
    protected abstract createDefaultObject(): T;

    /**
     * Registriert einen Client und erstellt ein BehaviorSubject mit leeren Attributen für diesen.
     * Achtung! Überschreibt schon registrierte Clients.
     */
    public registerClient(client: Client): void {
        this.clientMap.set(client, new BehaviorSubject<T>(
            this.createDefaultObject()
        ));
        console.log("Registered client " + client.name + " at " + this.constructor.name);
    }

    /**
     * Sendet ein Update für den Client.
     */
    public updateClient(client: Client): void {
        let entry = this.clientMap.get(client);
        if (entry) {
            entry.next(entry.value);
        } else {
            console.log("Client " + client.name + " is not registered!");
        }
    }

    /**
     * Gibt das BehaviorSubject für den Client zurück.
     * @param client
     */
    public getObservable(client: Client): Observable<T> {
        let entry = this.clientMap.get(client);
        if (entry) {
            return entry.asObservable();
        } else {
            console.log(this.clientMap);
            console.log("Client " + client.name + " is not registered! Returning empty BehaviorSubject.");
            return new BehaviorSubject<T>(this.createDefaultObject());
        }
    }

    /**
     * Gibt den Inhalt des BehaviorSubjects für den Client zurück, falls der Client registriert ist.
     */
    protected getValue(client: Client): T {
        let entry = this.clientMap.get(client);
        if (entry) {
            return entry.value;
        } else {
            console.error("Client " + client.name + " is not registered! Value could not be accessed.");
            return this.createDefaultObject();
        }
    }

    /**
     * Gibt eine Property aus dem Value des BehaviorSubjects zurück, falls der Client registriert ist.
     */
    protected getPropertyNullable<K extends keyof T>(client: Client, property: K): T[K] | null {
        let entry = this.clientMap.get(client);
        if (entry) {
            return entry.value[property];
        } else {
            console.error("Client " + client.name + " is not registered! Property " + String(property) + " could not be accessed.");
            return null;
        }
    }

    /**
     * Gibt eine Property aus dem Value des BehaviorSubjects als String zurück, falls der Client registriert ist.
     */
    protected getPropertyAsString<K extends keyof T>(client: Client, property: K): string {
        const value = this.getPropertyNullable(client, property);
        return value === null ? "" : String(value);
    }

    /**
     * Setzt eine Property im Value des BehaviorSubjects, falls der Client registriert ist.
     */
    protected setProperty<K extends keyof T>(client: Client, property: K, value: T[K]): void {
        let entry = this.clientMap.get(client);
        if (entry) {
            entry.value[property] = value;
        } else {
            console.error("Client " + client.name + " is not registered! Property " + String(property) + " could not be set.");
        }
    }

  /**
   * Entfernt die Registrierung eines Clients.
   */
  public unregisterClient(client: Client) {
    this.clientMap.delete(client);
  }
}
