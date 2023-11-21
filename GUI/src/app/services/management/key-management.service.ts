import {Injectable} from '@angular/core';
import {createEmptyKeyPair, KeyPair} from "../../models/key-pair";
import {Client} from "../../models/client";
import {ConfigurationData} from "../../models/configuration-data";
import {BackendRequestService} from "../backend-api/backend-request.service";
import {AbstractClientObservableManagementService} from './abstract-client-observable-management-service';
import {Observable} from "rxjs";

@Injectable({
  providedIn: 'root'
})
/**
 * Service zum Verwalten der Schlüssel von Clients.
 */
export class KeyManagementService extends AbstractClientObservableManagementService<KeyPair> {

  /**
   * @inheritDoc
   */
  protected override createDefaultObject(): KeyPair {
    return createEmptyKeyPair();
  }

  constructor(private backendRequestService: BackendRequestService) {
    super();
  }

  /**
   * Generiert ein Schlüsselpaar mit der gegebenen Konfiguration für den Client.
   * Nimmt ein Callback entgegen, welches nach dem Erhalt des Schlüsselpaares ausgeführt wird.
   */
  public generateKeyPair(requestContent: ConfigurationData, client: Client): Observable<void> {
    return new Observable<void>(observer => {
      this.backendRequestService.createKeyPair(requestContent).then(
        (keyPair) => {
          let entry = this.clientMap.get(client);
          if (entry) {
            entry.next(keyPair);
            observer.next();
            observer.complete();
          } else {
            // TODO im observer als Fehler werfen
            console.log("Client " + client + " is not registered!");
          }
        }
      );
    });
  }

  /**
   * Gibt das Schlüsselpaar des Clients zurück.
   */
  public getKeyPair(client: Client): KeyPair {
    return this.getValue(client);
  }

  /**
   * Setzt den Modul des Schlüssels vom Client.
   */
  public setModul(client: Client, modulus: string): void {
    this.setProperty(client, "modulus", modulus)
  }

  /**
   * Gibt den Modul des Schlüssels vom Client zurück.
   */
  public getModul(client: Client): string {
    return this.getPropertyAsString(client, "modulus");
  }

  /**
   * Setzt den öffentlichen Exponenten des Schlüssels vom Client.
   */
  public setE(client: Client, e: string): void {
    this.setProperty(client, "e", e);
  }

  /**
   * Gibt den öffentlichen Exponenten des Schlüssels vom Client zurück.
   */
  public getE(client: Client): string {
    return this.getPropertyAsString(client, "e");
  }

  /**
   * Setzt die Blockgröße für den öffentlichen Schlüssel des Clients.
   */
  public setBlockSizePub(client: Client, blockSize: string): void {
    this.setProperty(client, "block_size_pub", blockSize);
  }

  /**
   * Gibt die Blockgröße für den öffentlichen Schlüssel des Clients zurück.
   */
  public getBlockSizePub(client: Client): string {
    return this.getPropertyAsString(client, "block_size_pub");
  }

  /**
   * Setzt die Blockgröße für den privaten Schlüssel des Clients.
   */
  public setBlockSizePriv(client: Client, blockSize: string): void {
    this.setProperty(client, "block_size_priv", blockSize);
  }

  /**
   * Gibt die Blockgröße für den privaten Schlüssel des Clients zurück.
   */
  public getBlockSizePriv(client: Client): string {
    return this.getPropertyAsString(client, "block_size_priv");
  }

  /**
   * Setzt den privaten Exponenten des Schlüssels vom Client.
   */
  public setD(client: Client, d: string): void {
    this.setProperty(client, "d", d);
  }

  /**
   * Gibt den privaten Exponenten des Schlüssels vom Client zurück.
   */
  public getD(client: Client): string {
    return this.getPropertyAsString(client, "d");
  }
}
