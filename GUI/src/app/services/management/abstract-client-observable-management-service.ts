import {ClientEnum} from "../../models/client-enum";
import {BehaviorSubject} from "rxjs";

/**
 * Abstrakte Klasse, die die Verwaltung von Clients mit BehaviorSubjects übernimmt.
 */
export abstract class AbstractClientObservableManagementService<T> {

      protected clientMap = new Map<ClientEnum, BehaviorSubject<T>>();

      protected constructor() {
      }

      protected abstract createDefaultObject(): T;

      /**
       * Registriert einen Client und erstellt ein BehaviorSubject mit leeren Attributen für diesen.
       * @param client
       */
      public registerClient(client: ClientEnum): void {
        this.clientMap.set(client, new BehaviorSubject<T>(
          this.createDefaultObject()
        ));
      }

      /**
       * Gibt das BehaviorSubject für den Client zurück. Falls der Client noch nicht registriert ist, wird er registriert.
       * @param client
       */
      public getObservableWithRegister(client: ClientEnum): BehaviorSubject<T> {
        let entry = this.clientMap.get(client);
        if (entry) {
          return entry;
        } else {
          this.registerClient(client);
          return this.clientMap.get(client)!;
        }
      }

    /**
     * Gibt den Inhalt des BehaviorSubjects für den Client zurück, falls der Client registriert ist.
     */
    protected getValue(client: ClientEnum): T {
        let entry = this.clientMap.get(client);
        if (entry) {
            return entry.value;
        } else {
            console.error("Client " + client + " is not registered! Value could not be accessed.");
            return this.createDefaultObject();
        }
    }

    /**
     * Gibt eine Property aus dem Value des BehaviorSubjects zurück, falls der Client registriert ist.
     */
    protected getPropertyNullable<K extends keyof T>(client: ClientEnum, property: K): T[K] | null {
        let entry = this.clientMap.get(client);
        if (entry) {
            return entry.value[property];
        } else {
            console.error("Client " + client + " is not registered! Property " + String(property) + " could not be accessed.");
            return null;
        }
    }

    /**
     * Gibt eine Property aus dem Value des BehaviorSubjects als String zurück, falls der Client registriert ist.
     */
    protected getPropertyAsString<K extends keyof T>(client: ClientEnum, property: K): string {
        const value = this.getPropertyNullable(client, property);
        return value === null ? "" : String(value);
    }

    /**
     * Setzt eine Property im Value des BehaviorSubjects, falls der Client registriert ist.
     */
    protected setProperty<K extends keyof T>(client: ClientEnum, property: K, value: T[K]): void {
        let entry = this.clientMap.get(client);
        if (entry) {
            entry.value[property] = value;
        } else {
            console.error("Client " + client + " is not registered! Property " + String(property) + " could not be set.");
        }
    }

}
