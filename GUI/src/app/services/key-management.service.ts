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

  constructor() { }


  public generateKeyPair(requestContent: CreateKeyPairRequest,client: ClientEnum): void {
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
}
