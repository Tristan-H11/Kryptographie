import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class EndpointsService {

  private domain: string = "http://localhost:8080";
  private rsaEndpoint: string = "/rsa";

  constructor() { }

  public getHealthcheckEndpoint(): string {
    return this.domain + "/health";
  }

  public getCreateKeyPairEndpoint(): string {
    return this.domain + this.rsaEndpoint + "/createKeyPair";
  }

  public getEncryptEndpoint(): string {
    return this.domain + this.rsaEndpoint + "/encrypt";
  }

  public getDecryptEndpoint(): string {
    return this.domain + this.rsaEndpoint + "/decrypt";
  }

  public getSignEndpoint(): string {
    return this.domain + this.rsaEndpoint + "/sign";
  }

  public getVerifyEndpoint(): string {
    return this.domain + this.rsaEndpoint + "/verify";
  }

}
