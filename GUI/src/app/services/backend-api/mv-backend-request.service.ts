import {Injectable} from "@angular/core";
import {EndpointsService} from "./endpoints.service";
import {StateManagementService} from "../management/state-management.service";
import {HttpClient, HttpParams} from "@angular/common/http";
import {catchError, EMPTY, Observable} from "rxjs";
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
import {DialogService} from "../utility/dialogs.service";

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
        private http: HttpClient,
        private dialogService: DialogService
    ) {
    }

    /**
     * Fragt den Post Endpunkt zum Erstellen eines neuen Schlüsselpaares ab.
     * Dabei wird ein Fehler bereits abgefangen und im Dialog angezeigt.
     */
    public createKeyPair(body: MvKeygenConfig): Observable<MvKeyPair> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<MvKeyPair>(this.endpointsService.getMvCreateKeyPairEndpoint(), body, options);
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
     * Dabei wird ein Fehler bereits abgefangen und im Dialog angezeigt.
     */
    public encrypt(body: MvEncryptRequest): Observable<MvCipherText> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<MvCipherText>(this.endpointsService.getMvEncryptEndpoint(), body, options);
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
     * Fragt den Post Endpunkt zum Entschlüsseln einer Nachricht ab.
     * Dabei wird ein Fehler bereits abgefangen und im Dialog angezeigt.
     */
    public decrypt(body: MvDecryptRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<SingleMessageModel>(this.endpointsService.getMvDecryptEndpoint(), body, options);
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
     * Fragt den Post Endpunkt zum Signieren einer Nachricht ab.
     * Dabei wird ein Fehler bereits abgefangen und im Dialog angezeigt.
     */
    public sign(body: MvSignRequest): Observable<MvSignature> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<MvSignature>(this.endpointsService.getMvSignEndpoint(), body, options);
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
     * Fragt den Post Endpunkt zum Verifizieren einer Nachricht ab.
     * Dabei wird ein Fehler bereits abgefangen und im Dialog angezeigt.
     */
    public verify(body: MvVerifyRequest): Observable<SingleMessageModel> {
        const params = this.getParams();
        const options = {params};
        const response = this.http.post<SingleMessageModel>(this.endpointsService.getMvVerifyEndpoint(), body, options);
        return response.pipe(
            catchError(
                (error) => {
                    this.dialogService.showErrorDialog(error.error.message)
                    return EMPTY;
                }
            )
        );
    }

    private getParams(): HttpParams {
        return new HttpParams()
            .set("use_fast", this.stateService.getUseFastMath()());
    }
}
