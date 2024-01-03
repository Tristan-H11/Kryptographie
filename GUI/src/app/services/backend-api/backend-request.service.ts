import {Injectable} from "@angular/core";
import {HttpClient, HttpParams} from "@angular/common/http";
import {EndpointsService} from "./endpoints.service";
import {firstValueFrom, Observable} from "rxjs";
import {ConfigurationData} from "../../models/configuration-data";
import {KeyPair} from "../../models/key-pair";
import {EncryptDecryptRequest} from "../../models/encrypt-decrypt-request";
import {SingleMessageModel} from "../../models/SingleMessageModel";
import {SignRequest} from "../../models/sign-request";
import {VerifyRequest} from "../../models/verify-request";
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
 * Service zum Abfragen der Backend-Endpunkte.
 */
export class BackendRequestService {

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
     * Fragt den Post Endpunkt zum Ertellen eines neuen Schlüsselpaares ab.
     */
    public async createKeyPair(body: ConfigurationData): Promise<KeyPair> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<KeyPair>(this.endpointsService.getCreateKeyPairEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Verschlüsseln einer Nachricht ab.
     */
    public async encrypt(body: EncryptDecryptRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getEncryptEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Entschlüsseln einer Nachricht ab.
     */
    public async decrypt(body: EncryptDecryptRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getDecryptEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Signieren einer Nachricht ab.
     */
    public async sign(body: SignRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getSignEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Verifizieren einer Nachricht ab.
     */
    public async verify(body: VerifyRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getVerifyEndpoint(), body, options)
        );
    }

    public async exponentiation(body: ExponentiationRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getExponentiationEndpoint(), body, options)
        );
    }

    public async extendedGcd(body: ExtendedEuclidRequest): Promise<ExtendedEuclidResponse> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<ExtendedEuclidResponse>(this.endpointsService.getExtendedGcdEndpoint(), body, options)
        );
    }

    public async shanks(body: ShanksRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getShanksEndpoint(), body, options)
        );
    }

    private getParams(): HttpParams {
        return new HttpParams()
            .set("use_fast", this.stateService.getUseFastMath()());
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
}
