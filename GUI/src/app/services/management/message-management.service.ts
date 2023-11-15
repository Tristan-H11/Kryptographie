import {Injectable} from '@angular/core';
import {createEmptyMessageSignatureContainer, MessageSignatureContainer} from "../../models/message-signature-container";
import {ClientEnum} from "../../models/client-enum";
import {AbstractClientObservableManagementService} from "./abstract-client-observable-management-service";

@Injectable({
  providedIn: 'root'
})
export class MessageManagementService extends AbstractClientObservableManagementService<MessageSignatureContainer>{
  protected override createDefaultObject(): MessageSignatureContainer {
      return createEmptyMessageSignatureContainer();
  }

  constructor() {
      super();
  }

  public setPlaintext(plaintext: string, client: ClientEnum): void {
    this.setProperty(client, "plaintext", plaintext);
  }

  public getPlaintext(client: ClientEnum): string {
    return this.getPropertyAsString(client, "plaintext");
  }

  public setCiphertext(ciphertext: string, client: ClientEnum): void {
    this.setProperty(client, "ciphertext", ciphertext);
  }

  public getCiphertext(client: ClientEnum): string {
    return this.getPropertyAsString(client, "ciphertext");
  }

  public setSignature(signature: string, client: ClientEnum): void {
    this.setProperty(client, "signature", signature);
  }

  public getSignature(client: ClientEnum): string {
    return this.getPropertyAsString(client, "signature");
  }
}
