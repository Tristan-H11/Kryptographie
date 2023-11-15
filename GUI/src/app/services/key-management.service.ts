import {Injectable} from '@angular/core';
import {BehaviorSubject, Observable} from "rxjs";
import {createEmptyKeyPair, createKeyPairFrom, KeyPair} from "../models/key-pair";
import {ClientEnum} from "../models/client-enum";
import {CreateKeyPairRequest} from "../models/create-key-pair-request";

@Injectable({
  providedIn: 'root'
})
export class KeyManagementService {


  private aliceKeyPair = new BehaviorSubject<KeyPair>(
    createEmptyKeyPair()
  );
  private bobKeyPair = new BehaviorSubject<KeyPair>(
    createEmptyKeyPair()
  );

  constructor() {
  }


  public generateKeyPair(requestContent: CreateKeyPairRequest, client: ClientEnum): void {
    let keyPair: KeyPair = createKeyPairFrom(
      String(requestContent.modulus_width), //TODO
      String(requestContent.miller_rabin_rounds), //TODO
      "neuer WERT!", //TODO
      "blocksize enc", //TODO
      "blocksize dec" //TODO
    );

    keyPair.public_key.modulus = String(requestContent.modulus_width);
    keyPair.private_key.d = String(requestContent.miller_rabin_rounds);

    if (client == ClientEnum.Alice) {
      this.aliceKeyPair.next(keyPair);
    } else if (client == ClientEnum.Bob) {
      this.bobKeyPair.next(keyPair);
    }
  }

  public getKeyPair(client: ClientEnum): Observable<KeyPair> {
    if (client == ClientEnum.Alice) {
      return this.aliceKeyPair.asObservable();
    } else {
      return this.bobKeyPair.asObservable();
    }
  }

  public setModul(client: ClientEnum, modul: string): void {
    if (client == ClientEnum.Alice) {
      this.aliceKeyPair.value.public_key.modulus = modul;
    } else {
      this.bobKeyPair.value.public_key.modulus = modul;
    }
  }

  public getModul(client: ClientEnum) {
    if (client == ClientEnum.Alice) {
      return this.aliceKeyPair.value.public_key.modulus;
    } else {
      return this.bobKeyPair.value.public_key.modulus;
    }
  }

  public setE(client: ClientEnum, e: string): void {
    if (client == ClientEnum.Alice) {
      this.aliceKeyPair.value.public_key.e = e;
    } else {
      this.bobKeyPair.value.public_key.e = e;
    }
  }

  public getE(client: ClientEnum) {
    if (client == ClientEnum.Alice) {
      return this.aliceKeyPair.value.public_key.e;
    } else {
      return this.bobKeyPair.value.public_key.e;
    }
  }

  public getD(client: ClientEnum) {
    if (client == ClientEnum.Alice) {
      return this.aliceKeyPair.value.private_key.d;
    } else {
      return this.bobKeyPair.value.private_key.d;
    }
  }

  public setD(client: ClientEnum, d: string): void {
    if (client == ClientEnum.Alice) {
      this.aliceKeyPair.value.private_key.d = d;
    } else {
      this.bobKeyPair.value.private_key.d = d;
    }
  }
}
