import {Injectable} from "@angular/core";
import {EndpointsService} from "./endpoints.service";
import {StateManagementService} from "../management/state-management.service";
import {HttpClient, HttpParams} from "@angular/common/http";
import {firstValueFrom} from "rxjs";
import {SingleMessageModel} from "../../models/SingleMessageModel";
import {MvKeygenConfig} from "../../models/mv-keygen-config";
import {
    MvCipherText,
    MvDecryptRequest,
    MvEncryptRequest,
    MvKeyPair,
    MvSignature,
    MvSignRequest, MvVerifyRequest
} from "../../models/mv-beans";

@Injectable({
    providedIn: "root"
})
/**
 * Service zum Abfragen der RSA Backend-Endpunkte.
 */
export class MvBackendRequestService {
    constructor(
        private endpointsService: EndpointsService,
        private stateService: StateManagementService,
        private http: HttpClient
    ) {
    }

    /**
     * Fragt den Post Endpunkt zum Erstellen eines neuen Schlüsselpaares ab.
     */
    public async createKeyPair(body: MvKeygenConfig): Promise<MvKeyPair> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<MvKeyPair>(this.endpointsService.getMvCreateKeyPairEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Verschlüsseln einer Nachricht ab.
     */
    public async encrypt(body: MvEncryptRequest): Promise<MvCipherText> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<MvCipherText>(this.endpointsService.getMvEncryptEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Entschlüsseln einer Nachricht ab.
     */
    public async decrypt(body: MvDecryptRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getMvDecryptEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Signieren einer Nachricht ab.
     */
    public async sign(body: MvSignRequest): Promise<MvSignature> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<MvSignature>(this.endpointsService.getMvSignEndpoint(), body, options)
        );
    }

    /**
     * Fragt den Post Endpunkt zum Verifizieren einer Nachricht ab.
     */
    public async verify(body: MvVerifyRequest): Promise<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        return firstValueFrom(
            this.http.post<SingleMessageModel>(this.endpointsService.getMvVerifyEndpoint(), body, options)
        );
    }

    private getParams(): HttpParams {
        return new HttpParams()
            .set("use_fast", this.stateService.getUseFastMath()());
    }
}
