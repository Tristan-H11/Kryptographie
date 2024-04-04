import {Injectable} from "@angular/core";
import {HttpClient, HttpParams} from "@angular/common/http";
import {EndpointsService} from "./endpoints.service";
import {firstValueFrom, Observable} from "rxjs";
import {RsaConfigurationData} from "../../models/rsa-configuration-data";
import {RsaKeyPair} from "../../models/rsa-key-pair";
import {RsaEncryptDecryptRequest} from "../../models/rsa-encrypt-decrypt-request";
import {SingleMessageModel} from "../../models/SingleMessageModel";
import {RsaSignRequest} from "../../models/rsa-sign-request";
import {RsaVerifyRequest} from "../../models/rsa-verify-request";
import {ExponentiationRequest} from "../../models/exponentiation-request";
import {ExtendedEuclidRequest} from "../../models/extended-euclid-request";
import {ExtendedEuclidResponse} from "../../models/extended-euclid-response";
import {StateManagementService} from "../management/state-management.service";
import {ShanksRequest} from "../../models/shanks-request";
import {ModularInversRequest} from "../../models/modular-invers-request";
import {MultiplicationRequest} from "../../models/multiplication-request";
import {MultiplicationResponse} from "../../models/multiplication-response";

@Injectable({
    providedIn: "root"
})
/**
 * Service zum Abfragen der RSA Backend-Endpunkte.
 */
export class RsaBackendRequestService {

    constructor(
        private endpointsService: EndpointsService,
        private stateService: StateManagementService,
        private http: HttpClient
    ) {
    }

    /**
     * Fragt den Healthcheck-Endpoint ab und gibt zurück, ob der Server erreichbar ist.
     */
    public async checkHealth(): Promise<boolean> {
        try {
            await firstValueFrom(
                this.http.get(this.endpointsService.getHealthcheckEndpoint())
            );
            return true;
        } catch {
            return false;
        }
    }

    /**
     * Fragt den Post Endpunkt zum Erstellen eines neuen Schlüsselpaares ab.
     */
    public async createKeyPair(body: RsaConfigurationData): Promise<RsaKeyPair> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<RsaKeyPair>(this.endpointsService.getRsaCreateKeyPairEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Verschlüsseln einer Nachricht ab.
     */
    public async encrypt(body: RsaEncryptDecryptRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getRsaEncryptEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Entschlüsseln einer Nachricht ab.
     */
    public async decrypt(body: RsaEncryptDecryptRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getRsaDecryptEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Signieren einer Nachricht ab.
     */
    public async sign(body: RsaSignRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getRsaSignEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Verifizieren einer Nachricht ab.
     */
    public async verify(body: RsaVerifyRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getRsaVerifyEndpoint(), body, options)
        );
    }

    public exponentiation(body: ExponentiationRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return this.http.post<SingleMessageModel>(this.endpointsService.getExponentiationEndpoint(), body, options);
    }

    public extendedGcd(body: ExtendedEuclidRequest): Observable<ExtendedEuclidResponse> {
        const params = this.getParams();
        const options = {params};
        return this.http.post<ExtendedEuclidResponse>(this.endpointsService.getExtendedGcdEndpoint(), body, options);
    }

    public shanks(body: ShanksRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return this.http.post<SingleMessageModel>(this.endpointsService.getShanksEndpoint(), body, options);
    }

    public modularInverse(body: ModularInversRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return this.http.post<SingleMessageModel>(this.endpointsService.getModularInverseEndpoint(), body, options);
    }

    public rsaMultiplication(body: MultiplicationRequest): Observable<MultiplicationResponse> {
        const params = this.getParams();
        const options = {params};
        return this.http.post<MultiplicationResponse>(this.endpointsService.getRsaMultiplicationEndpoint(), body, options);
    }

    private getParams(): HttpParams {
        return new HttpParams()
            .set("use_fast", this.stateService.getUseFastMath()());
    }
}
