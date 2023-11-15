import {ClientEnum} from "../models/client-enum";
import {BehaviorSubject} from "rxjs";

/**
 * Abstrakte Klasse, die die Verwaltung von Clients mit BehaviorSubjects übernimmt.
 */
export abstract class AbstractClientObservableManagementService<T> {

      protected clientMap = new Map<ClientEnum, BehaviorSubject<T>>();

      protected constructor() {
      }

      protected abstract createEmptyObject(): T;

      /**
       * Registriert einen Client und erstellt ein BehaviorSubject mit leeren Attributen für diesen.
       * @param client
       */
      public registerClient(client: ClientEnum): void {
        this.clientMap.set(client, new BehaviorSubject<T>(
          this.createEmptyObject()
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

}
