import {Injectable} from '@angular/core';
import {createEmptyMessageSignatureContainer, MessageSignatureContainer} from "../../models/message-signature-container";
import {ClientEnum} from "../../models/client-enum";
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
  public setPlaintext(plaintext: string, client: ClientEnum): void {
    this.setProperty(client, "plaintext", plaintext);
  }

  /**
   * Gibt den Klartext des Clients zurück.
   */
  public getPlaintext(client: ClientEnum): string {
    return this.getPropertyAsString(client, "plaintext");
  }

  /**
   * Setzt den Geheimtext des Clients.
   */
  public setCiphertext(ciphertext: string, client: ClientEnum): void {
    this.setProperty(client, "ciphertext", ciphertext);
  }

  /**
   * Gibt den Geheimtext des Clients zurück.
   */
  public getCiphertext(client: ClientEnum): string {
    return this.getPropertyAsString(client, "ciphertext");
  }

  /**
   * Setzt die Signatur des Clients.
   */
  public setSignature(signature: string, client: ClientEnum): void {
    this.setProperty(client, "signature", signature);
  }

  /**
   * Gibt die Signatur des Clients zurück.
   */
  public getSignature(client: ClientEnum): string {
    return this.getPropertyAsString(client, "signature");
  }
}
