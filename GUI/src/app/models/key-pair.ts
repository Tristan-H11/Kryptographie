import {PublicKey} from "./public-key";
import {PrivateKey} from "./private-key";

export interface KeyPair {
  public_key: PublicKey;
  private_key: PrivateKey;
}
