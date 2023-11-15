import {Injectable} from '@angular/core';
import {BehaviorSubject, Observable} from "rxjs";
import {KeyPair} from "../models/key-pair";
import {ClientEnum} from "../models/client-enum";
import {CreateKeyPairRequest} from "../models/create-key-pair-request";

@Injectable({
  providedIn: 'root'
})
export class KeyManagementService {

  private emptyKeyPair: KeyPair = {
    public_key: {
      modulus: "",
      e: "",
      block_size: ""
    },
    private_key: {
      modulus: "",
      d: "",
      block_size: ""
    }
  };

  private aliceKeyPair = new BehaviorSubject<KeyPair>(this.emptyKeyPair);
  private bobKeyPair = new BehaviorSubject<KeyPair>(this.emptyKeyPair);

  constructor() { }


  private generateKeyPair(requestContent: CreateKeyPairRequest,client: ClientEnum): void {
    let keyPair: KeyPair = this.emptyKeyPair;

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
