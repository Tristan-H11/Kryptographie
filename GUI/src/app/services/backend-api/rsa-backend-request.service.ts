import {Injectable} from "@angular/core";
import {HttpClient, HttpParams} from "@angular/common/http";
import {EndpointsService} from "./endpoints.service";
import {catchError, EMPTY, firstValueFrom, Observable} from "rxjs";
import {RsaCreateKeyPairRequest} from "../../models/rsa-create-key-pair-request";
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
import {DialogService} from "../utility/dialogs.service";
import {ShanksResponse} from "../../models/ShanksResponse";

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
        private http: HttpClient,
        private dialogService: DialogService
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
     * Dabei wird ein Error schon abgefangen und in einem Dialog angezeigt.
     */
    public createKeyPair(body: RsaCreateKeyPairRequest): Observable<RsaKeyPair> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<RsaKeyPair>(this.endpointsService.getRsaCreateKeyPairEndpoint(), body, options);
        return response.pipe(
            catchError(
                    (error) => {
                        this.dialogService.showErrorDialog(error.error.message)
                        return EMPTY;
                }
            )
        );
    }

    /**
     * Fragt den Post Endpunkt zum Verschlüsseln einer Nachricht ab.
     * Dabei wird ein Error schon abgefangen und in einem Dialog angezeigt.
     */
    public encrypt(body: RsaEncryptDecryptRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<SingleMessageModel>(this.endpointsService.getRsaEncryptEndpoint(), body, options);
        return response.pipe(
            catchError(
                (error) => {
                    this.dialogService.showErrorDialog(error.error.message)
                    return EMPTY;
                }
            )
        );;
    }

    /**
     * Fragt den Post Endpunkt zum Entschlüsseln einer Nachricht ab.
     * Dabei wird ein Error schon abgefangen und in einem Dialog angezeigt.
     */
    public decrypt(body: RsaEncryptDecryptRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<SingleMessageModel>(this.endpointsService.getRsaDecryptEndpoint(), body, options);
        return response.pipe(
            catchError(
                (error) => {
                    this.dialogService.showErrorDialog(error.error.message)
                    return EMPTY;
                }
            )
        );;
    }

    /**
     * Fragt den Post Endpunkt zum Signieren einer Nachricht ab.
     * Dabei wird ein Error schon abgefangen und in einem Dialog angezeigt.
     */
    public sign(body: RsaSignRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<SingleMessageModel>(this.endpointsService.getRsaSignEndpoint(), body, options);
        return response.pipe(
            catchError(
                (error) => {
                    this.dialogService.showErrorDialog(error.error.message)
                    return EMPTY;
                }
            )
        );;
    }

    /**
     * Fragt den Post Endpunkt zum Verifizieren einer Nachricht ab.
     * Dabei wird ein Error schon abgefangen und in einem Dialog angezeigt.
     */
    public verify(body: RsaVerifyRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<SingleMessageModel>(this.endpointsService.getRsaVerifyEndpoint(), body, options);
        return response.pipe(
            catchError(
                (error) => {
                    this.dialogService.showErrorDialog(error.error.message)
                    return EMPTY;
                }
            )
        );;
    }

    public exponentiation(body: ExponentiationRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<SingleMessageModel>(this.endpointsService.getExponentiationEndpoint(), body, options);
        return response.pipe(
            catchError(
                (error) => {
                    this.dialogService.showErrorDialog(error.error.message)
                    return EMPTY;
                }
            )
        );
    }

    public extendedGcd(body: ExtendedEuclidRequest): Observable<ExtendedEuclidResponse> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<ExtendedEuclidResponse>(this.endpointsService.getExtendedGcdEndpoint(), body, options);
        return response.pipe(
            catchError(
                (error) => {
                    this.dialogService.showErrorDialog(error.error.message)
                    return EMPTY;
                }
            )
        );
    }

    public shanks(body: ShanksRequest): Observable<ShanksResponse> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<ShanksResponse>(this.endpointsService.getShanksEndpoint(), body, options);
        return response.pipe(
            catchError(
                (error) => {
                    this.dialogService.showErrorDialog(error.error.message)
                    return EMPTY;
                }
            )
        );
    }

    public modularInverse(body: ModularInversRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<SingleMessageModel>(this.endpointsService.getModularInverseEndpoint(), body, options);
        return response.pipe(
            catchError(
                (error) => {
                    this.dialogService.showErrorDialog(error.error.message)
                    return EMPTY;
                }
            )
        );
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
