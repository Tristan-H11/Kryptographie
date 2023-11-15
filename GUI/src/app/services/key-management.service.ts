import {Injectable} from '@angular/core';
import {BehaviorSubject, Observable} from "rxjs";
import {createEmptyKeyPair, createKeyPairFrom, KeyPair} from "../models/key-pair";
import {ClientEnum} from "../models/client-enum";
import {CreateKeyPairRequest} from "../models/create-key-pair-request";

@Injectable({
  providedIn: 'root'
})
export class KeyManagementService {


  private keyMap = new Map<ClientEnum, BehaviorSubject<KeyPair>>();

  constructor() {
  }

  /**
   * Registriert einen Client und erstellt ein BehaviorSubject mit leeren Attributen für diesen.
   * @param client
   */
  public registerClient(client: ClientEnum): void {
    this.keyMap.set(client, new BehaviorSubject<KeyPair>(
      createEmptyKeyPair()
    ));
  }

  /**
   * Gibt das BehaviorSubject für den Client zurück. Falls der Client noch nicht registriert ist, wird er registriert.
   * @param client
   */
  public getKeyPairObservableWithRegister(client: ClientEnum): Observable<KeyPair> {
    let entry = this.keyMap.get(client);
    if (entry) {
      return entry.asObservable();
    } else {
      this.registerClient(client);
      return this.keyMap.get(client)!.asObservable();
    }
  }


  /*
  TODO Das muss in RSA Service oder so. denke ich. Wir werden sehen..
   */
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

    let entry = this.keyMap.get(client);
    if (entry) {
      entry.next(keyPair);
    } else {
      console.log("Client " + client + " is not registered!");
    }
  }


  public setModul(client: ClientEnum, modulus: string): void {
    let entry = this.keyMap.get(client);
    if (entry) {
      entry.value.public_key.modulus = modulus;
    } else {
      console.log("Client " + client + " is not registered!");
    }
  }

  public getModul(client: ClientEnum) {
    let entry = this.keyMap.get(client);
    if (entry) {
      return entry.value.public_key.modulus;
    } else {
      console.log("Client " + client + " is not registered!");
      return "";
    }
  }

  public setE(client: ClientEnum, e: string): void {
    let entry = this.keyMap.get(client);
    if (entry) {
      entry.value.public_key.e = e;
    } else {
      console.log("Client " + client + " is not registered!");
    }
  }

  public getE(client: ClientEnum) {
    let entry = this.keyMap.get(client);
    if (entry) {
      return entry.value.public_key.e;
    } else {
      console.log("Client " + client + " is not registered!");
      return "";
    }
  }

  public getD(client: ClientEnum) {
    let entry = this.keyMap.get(client);
    if (entry) {
      return entry.value.private_key.d;
    } else {
      console.log("Client " + client + " is not registered!");
      return "";
    }
  }

  public setD(client: ClientEnum, d: string): void {
    let entry = this.keyMap.get(client);
    if (entry) {
      entry.value.private_key.d = d;
    } else {
      console.log("Client " + client + " is not registered!");
    }
  }
}
