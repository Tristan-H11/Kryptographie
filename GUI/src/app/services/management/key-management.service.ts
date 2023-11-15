import {Injectable} from '@angular/core';
import {createEmptyKeyPair, KeyPair} from "../../models/key-pair";
import {ClientEnum} from "../../models/client-enum";
import {ConfigurationData} from "../../models/configuration-data";
import {BackendRequestService} from "../backend-request.service";
import {AbstractClientObservableManagementService} from './abstract-client-observable-management-service';

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
   */
  public generateKeyPair(requestContent: ConfigurationData, client: ClientEnum): void {
    this.backendRequestService.createKeyPair(requestContent).then(
      (keyPair) => {
        let entry = this.clientMap.get(client);
        if (entry) {
          entry.next(keyPair);
        } else {
          console.log("Client " + client + " is not registered!");
        }
      }
    );
  }

  /**
   * Gibt das Schlüsselpaar des Clients zurück.
   */
  public getKeyPair(client: ClientEnum): KeyPair {
    return this.getValue(client);
  }

  /**
   * Setzt den Modul des Schlüssels vom Client.
   */
  public setModul(client: ClientEnum, modulus: string): void {
    this.setProperty(client, "modulus", modulus)
  }

  /**
   * Gibt den Modul des Schlüssels vom Client zurück.
   */
  public getModul(client: ClientEnum): string {
    return this.getPropertyAsString(client, "modulus");
  }

  /**
   * Setzt den öffentlichen Exponenten des Schlüssels vom Client.
   */
  public setE(client: ClientEnum, e: string): void {
    this.setProperty(client, "e", e);
  }

  /**
   * Gibt den öffentlichen Exponenten des Schlüssels vom Client zurück.
   */
  public getE(client: ClientEnum): string {
    return this.getPropertyAsString(client, "e");
  }

  /**
   * Setzt die Blockgröße für den öffentlichen Schlüssel des Clients.
   */
  public setBlockSizePub(client: ClientEnum, blockSize: string): void {
    this.setProperty(client, "block_size_pub", blockSize);
  }

  /**
   * Gibt die Blockgröße für den öffentlichen Schlüssel des Clients zurück.
   */
  public getBlockSizePub(client: ClientEnum): string {
    return this.getPropertyAsString(client, "block_size_pub");
  }

  /**
   * Setzt die Blockgröße für den privaten Schlüssel des Clients.
   */
  public setBlockSizePriv(client: ClientEnum, blockSize: string): void {
    this.setProperty(client, "block_size_priv", blockSize);
  }

  /**
   * Gibt die Blockgröße für den privaten Schlüssel des Clients zurück.
   */
  public getBlockSizePriv(client: ClientEnum): string {
    return this.getPropertyAsString(client, "block_size_priv");
  }

  /**
   * Setzt den privaten Exponenten des Schlüssels vom Client.
   */
  public setD(client: ClientEnum, d: string): void {
    this.setProperty(client, "d", d);
  }

  /**
   * Gibt den privaten Exponenten des Schlüssels vom Client zurück.
   */
  public getD(client: ClientEnum): string {
    return this.getPropertyAsString(client, "d");
  }
}
