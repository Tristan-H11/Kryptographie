import {Injectable} from '@angular/core';
import {createEmptyMessageSignatureContainer, MessageSignatureContainer} from "../../models/message-signature-container";
import {Client} from "../../models/client";
import {AbstractClientObservableManagementService} from "./abstract-client-observable-management-service";

@Injectable({
  providedIn: 'root'
})
/**
 * Service zum Verwalten des Klartextes, Geheimtextes und der Signatur von Clients.
 */
export class MessageManagementService extends AbstractClientObservableManagementService<MessageSignatureContainer>{
  /**
   * @inheritDoc
   */
  protected override createDefaultObject(): MessageSignatureContainer {
      return createEmptyMessageSignatureContainer();
  }

  constructor() {
      super();
  }

  /**
   * Setzt den Klartext des Clients.
   */
  public setPlaintext(plaintext: string, client: Client): void {
    this.setProperty(client, "plaintext", plaintext);
  }

  /**
   * Gibt den Klartext des Clients zurück.
   */
  public getPlaintext(client: Client): string {
    return this.getPropertyAsString(client, "plaintext");
  }

  /**
   * Setzt den Geheimtext des Clients.
   */
  public setCiphertext(ciphertext: string, client: Client): void {
    this.setProperty(client, "ciphertext", ciphertext);
  }

  /**
   * Gibt den Geheimtext des Clients zurück.
   */
  public getCiphertext(client: Client): string {
    return this.getPropertyAsString(client, "ciphertext");
  }

  /**
   * Setzt die Signatur des Clients.
   */
  public setSignature(signature: string, client: Client): void {
    this.setProperty(client, "signature", signature);
  }

  /**
   * Gibt die Signatur des Clients zurück.
   */
  public getSignature(client: Client): string {
    return this.getPropertyAsString(client, "signature");
  }
}
