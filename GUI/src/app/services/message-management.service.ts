import { Injectable } from '@angular/core';
import {createEmptyMessageSignatureContainer, MessageSignatureContainer} from "../models/message-signature-container";
import {BehaviorSubject, Observable} from "rxjs";
import {ClientEnum} from "../models/client-enum";

@Injectable({
  providedIn: 'root'
})
export class MessageManagementService {

  private messageMap = new Map<ClientEnum, BehaviorSubject<MessageSignatureContainer>>();

  constructor() { }

  /**
   * Registriert einen Client und erstellt ein BehaviorSubject mit leeren Attributen für diesen.
   * @param client
   */
  public registerClient(client: ClientEnum): void {
    this.messageMap.set(client, new BehaviorSubject<MessageSignatureContainer>(
      createEmptyMessageSignatureContainer()
    ));
  }

  /**
   * Gibt das BehaviorSubject für den Client zurück. Falls der Client noch nicht registriert ist, wird er registriert.
   * @param client
   */
  public getMessageObservableWithRegister(client: ClientEnum): Observable<MessageSignatureContainer> {
    let entry = this.messageMap.get(client);
    if (entry) {
      return entry.asObservable();
    } else {
      this.registerClient(client);
      return this.messageMap.get(client)!.asObservable();
    }
  }

  public setPlaintext(plaintext: string, client: ClientEnum): void {
    let entry = this.messageMap.get(client);
    if (entry) {
      entry.value.plaintext = plaintext;
    } else {
      console.log("Client " + client + " is not registered!");
    }
  }

  public getPlaintext(client: ClientEnum): string {
    let entry = this.messageMap.get(client);
    if (entry) {
      return entry.value.plaintext;
    } else {
      console.log("Client " + client + " is not registered!");
      return "";
    }
  }

  public setCiphertext(ciphertext: string, client: ClientEnum): void {
    let entry = this.messageMap.get(client);
    if (entry) {
      entry.value.ciphertext = ciphertext;
    } else {
      console.log("Client " + client + " is not registered!");
    }
  }

  public getCiphertext(client: ClientEnum): string {
    let entry = this.messageMap.get(client);
    if (entry) {
      return entry.value.ciphertext;
    } else {
      console.log("Client " + client + " is not registered!");
      return "";
    }
  }

  public setSignature(signature: string, client: ClientEnum): void {
    let entry = this.messageMap.get(client);
    if (entry) {
      entry.value.signature = signature;
    } else {
      console.log("Client " + client + " is not registered!");
    }
  }

  public getSignature(client: ClientEnum): string {
    let entry = this.messageMap.get(client);
    if (entry) {
      return entry.value.signature;
    } else {
      console.log("Client " + client + " is not registered!");
      return "";
    }
  }
}
