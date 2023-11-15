import { Injectable } from '@angular/core';
import {createEmptyMessageSignature, MessageSignature} from "../models/message-signature";
import {BehaviorSubject, Observable} from "rxjs";
import {ClientEnum} from "../models/client-enum";

@Injectable({
  providedIn: 'root'
})
export class MessageManagementService {

  private aliceMessage = new BehaviorSubject<MessageSignature>(
    createEmptyMessageSignature()
  );
  private bobMessage = new BehaviorSubject<MessageSignature>(
    createEmptyMessageSignature()
  );

  constructor() { }

  public setPlaintext(plaintext: string, client: ClientEnum): void {
    if (client == ClientEnum.Alice) {
      this.aliceMessage.value.plaintext = plaintext;
    } else if (client == ClientEnum.Bob) {
      this.bobMessage.value.plaintext = plaintext;
    }
  }

  public getPlaintext(client: ClientEnum): string {
    if (client == ClientEnum.Alice) {
      return this.aliceMessage.value.plaintext;
    } else {
      return this.bobMessage.value.plaintext;
    }
  }

  public setCiphertext(ciphertext: string, client: ClientEnum): void {
    if (client == ClientEnum.Alice) {
      this.aliceMessage.value.ciphertext = ciphertext;
    } else if (client == ClientEnum.Bob) {
      this.bobMessage.value.ciphertext = ciphertext;
    }
  }

  public getCiphertext(client: ClientEnum): string {
    if (client == ClientEnum.Alice) {
      return this.aliceMessage.value.ciphertext;
    } else {
      return this.bobMessage.value.ciphertext;
    }
  }

  public setSignature(signature: string, client: ClientEnum): void {
    if (client == ClientEnum.Alice) {
      this.aliceMessage.value.signature = signature;
    } else if (client == ClientEnum.Bob) {
      this.bobMessage.value.signature = signature;
    }
  }

  public getSignature(client: ClientEnum): string {
    if (client == ClientEnum.Alice) {
      return this.aliceMessage.value.signature;
    } else {
      return this.bobMessage.value.signature;
    }
  }

  public getMessageOberservable(client: ClientEnum): Observable<MessageSignature> {
    if (client == ClientEnum.Alice) {
      return this.aliceMessage.asObservable();
    } else {
      return this.bobMessage.asObservable();
    }
  }
}
