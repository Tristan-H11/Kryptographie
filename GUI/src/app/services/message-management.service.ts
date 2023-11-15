import {Injectable} from '@angular/core';
import {createEmptyMessageSignatureContainer, MessageSignatureContainer} from "../models/message-signature-container";
import {ClientEnum} from "../models/client-enum";
import {AbstractClientObservableManagementService} from "./abstract-client-observable-management-service";

@Injectable({
  providedIn: 'root'
})
export class MessageManagementService extends AbstractClientObservableManagementService<MessageSignatureContainer>{
  protected override createEmptyObject(): MessageSignatureContainer {
      return createEmptyMessageSignatureContainer();
  }

  constructor() {
      super();
  }

  public setPlaintext(plaintext: string, client: ClientEnum): void {
    let entry = this.clientMap.get(client);
    if (entry) {
      entry.value.plaintext = plaintext;
    } else {
      console.log("Client " + client + " is not registered!");
    }
  }

  public getPlaintext(client: ClientEnum): string {
    let entry = this.clientMap.get(client);
    if (entry) {
      return entry.value.plaintext;
    } else {
      console.log("Client " + client + " is not registered!");
      return "";
    }
  }

  public setCiphertext(ciphertext: string, client: ClientEnum): void {
    let entry = this.clientMap.get(client);
    if (entry) {
      entry.value.ciphertext = ciphertext;
    } else {
      console.log("Client " + client + " is not registered!");
    }
  }

  public getCiphertext(client: ClientEnum): string {
    let entry = this.clientMap.get(client);
    if (entry) {
      return entry.value.ciphertext;
    } else {
      console.log("Client " + client + " is not registered!");
      return "";
    }
  }

  public setSignature(signature: string, client: ClientEnum): void {
    let entry = this.clientMap.get(client);
    if (entry) {
      entry.value.signature = signature;
    } else {
      console.log("Client " + client + " is not registered!");
    }
  }

  public getSignature(client: ClientEnum): string {
    let entry = this.clientMap.get(client);
    if (entry) {
      return entry.value.signature;
    } else {
      console.log("Client " + client + " is not registered!");
      return "";
    }
  }
}
