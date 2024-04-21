import {Component, Input} from "@angular/core";
import {ClientActionRowComponent} from "../../shared/client-action-row/client-action-row.component";
import {EmptyIfUndefinedPipe} from "../../../services/pipes/empty-if-undefined";
import {MatCard, MatCardContent, MatCardHeader, MatCardTitle} from "@angular/material/card";
import {
    MatExpansionPanel,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatFormField, MatLabel} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {RsaClientData} from "../../shared/ClientData";
import {RsaConfigurationData} from "../../shared/ConfigurationDataTypes";
import {DialogService} from "../../../services/utility/dialogs.service";
import {RsaBackendRequestService} from "../../../services/backend-api/rsa-backend-request.service";
import {RsaEncryptDecryptRequest} from "../../../models/rsa-encrypt-decrypt-request";
import {RsaSignRequest} from "../../../models/rsa-sign-request";
import {RsaVerifyRequest} from "../../../models/rsa-verify-request";
import {AbstractClientPanelComponent} from "../../shared/AbstractClientPanelComponent";
import {concatMap, EMPTY} from "rxjs";

@Component({
    selector: "rsa-client-panel",
    standalone: true,
    imports: [
        ClientActionRowComponent,
        EmptyIfUndefinedPipe,
        MatCard,
        MatCardContent,
        MatCardHeader,
        MatCardTitle,
        MatExpansionPanel,
        MatExpansionPanelDescription,
        MatExpansionPanelHeader,
        MatExpansionPanelTitle,
        MatFormField,
        MatInput,
        MatLabel,
        ReactiveFormsModule,
        FormsModule
    ],
    templateUrl: "./rsa-client-panel.component.html",
})
export class RsaClientPanelComponent extends AbstractClientPanelComponent<RsaClientData> {
    protected override createDefaultClient(name: string): RsaClientData {
        return RsaClientData.createDefaultWithName(name);
    }

    @Input()
    public config: RsaConfigurationData = {
        modulusWidth: 0,
        millerRabinRounds: 0,
        randomSeed: 0,
        numberSystem: 0
    };

    constructor(private backendRequestService: RsaBackendRequestService,
                private dialogService: DialogService) {
        super();
    }

    /**
     * Verschlüsselt und Signiert die Nachricht für das gewählte Ziel.
     */
    public encrypt(): void {
        if (!this.client.sendingTo || !this.client.sendingTo.keyPair) {
            return;
        }
        const requestBody = new RsaEncryptDecryptRequest(
            this.client.plaintext,
            this.client.sendingTo.keyPair,
            this.config.numberSystem
        );

        let loadingCalcKey = this.dialogService.startTimedCalc();

        this.backendRequestService.encrypt(requestBody).pipe(
            concatMap(response => {
                this.client.ciphertext = response.message;

                if (!this.client.keyPair) {
                    this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht verschlüsselt.");
                    return EMPTY;
                }

                const signRequest = new RsaSignRequest(
                    this.client.plaintext,
                    this.client.keyPair,
                    this.config.numberSystem
                );
                return this.backendRequestService.sign(signRequest);
            })
        ).subscribe(response => {
            this.client.signature = response.message;
            this.client.signature_valid = "ungeprüft";
            this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht verschlüsselt und signiert.");
        });
    }

    /**
     * Verschlüsselt und Signiert die Nachricht für das gewählte Ziel.
     */
    public decrypt(): void {
        if (!this.client.receivedFrom || !this.client.keyPair) {
            return;
        }
        const requestBody = new RsaEncryptDecryptRequest(
            this.client.ciphertext,
            this.client.keyPair,
            this.config.numberSystem
        );

        let loadingCalcKey = this.dialogService.startTimedCalc();

        this.backendRequestService.decrypt(requestBody).pipe(
            concatMap(response => {
                this.client.plaintext = response.message;

                if (!this.client.receivedFrom || !this.client.receivedFrom.keyPair) {
                    this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht entschlüsselt.");
                    return EMPTY;
                }

                const verifyRequest = new RsaVerifyRequest(
                    this.client.plaintext,
                    this.client.signature,
                    this.client.receivedFrom.keyPair,
                    this.config.numberSystem
                );
                return this.backendRequestService.verify(verifyRequest);
            })
        ).subscribe(response => {
            if (response.message === "true") {
                this.client.signature_valid = "gültig";
            } else {
                this.client.signature_valid = "ungültig";
            }
            this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht entschlüsselt und verifiziert.");
        });
    }
}
