import {Injectable} from "@angular/core";
import {HttpClient, HttpParams} from "@angular/common/http";
import {catchError, EMPTY, Observable} from "rxjs";
import {EndpointsService} from "./endpoints.service";
import {StateManagementService} from "../management/state-management.service";
import {DialogService} from "../utility/dialogs.service";
import {GghDecryptRequest, GghEncryptRequest, GghKeyPairBean, GghVectorBean} from "../../models/ggh-beans";

export interface GghCreateKeyPairRequest {
    dimension: number;
    basis_vector_length: number;
    unimodular_iterations: number;
    random_seed: number;
}

@Injectable({
    providedIn: "root"
})
export class GghBackendRequestService {

    constructor(
        private endpointsService: EndpointsService,
        private stateService: StateManagementService,
        private http: HttpClient,
        private dialogService: DialogService
    ) {
    }

    public createKeyPair(body: GghCreateKeyPairRequest): Observable<GghKeyPairBean> {
        const params = this.getParams();
        const response = this.http.post<GghKeyPairBean>(
            this.endpointsService.getGghCreateKeyPairEndpoint(), body, {params}
        );
        return response.pipe(
            catchError((error) => {
                this.dialogService.showErrorDialog(error.error.message);
                return EMPTY;
            })
        );
    }

    public encrypt(body: GghEncryptRequest): Observable<GghVectorBean> {
        const params = this.getParams();
        const response = this.http.post<GghVectorBean>(
            this.endpointsService.getGghEncryptEndpoint(), body, {params}
        );
        return response.pipe(
            catchError((error) => {
                this.dialogService.showErrorDialog(error.error.message);
                return EMPTY;
            })
        );
    }

    public decrypt(body: GghDecryptRequest): Observable<GghVectorBean> {
        const params = this.getParams();
        const response = this.http.post<GghVectorBean>(
            this.endpointsService.getGghDecryptEndpoint(), body, {params}
        );
        return response.pipe(
            catchError((error) => {
                this.dialogService.showErrorDialog(error.error.message);
                return EMPTY;
            })
        );
    }

    private getParams(): HttpParams {
        return new HttpParams()
            .set("use_fast", this.stateService.getUseFastMath()());
    }
}
