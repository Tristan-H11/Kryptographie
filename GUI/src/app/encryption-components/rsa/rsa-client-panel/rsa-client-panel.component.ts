import {Component, EventEmitter, Input, Output} from "@angular/core";
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
import {createDefaultRsaClientData, RsaClientData} from "../../shared/IClientData";
import {RsaConfigurationData} from "../../shared/ConfigurationDataTypes";
import {DialogService} from "../../../services/utility/dialogs.service";
import {RsaBackendRequestService} from "../../../services/backend-api/rsa-backend-request.service";
import {RsaEncryptDecryptRequest} from "../../../models/rsa-encrypt-decrypt-request";
import {RsaSignRequest} from "../../../models/rsa-sign-request";
import {RsaVerifyRequest} from "../../../models/rsa-verify-request";

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
    styleUrl: "./rsa-client-panel.component.scss"
})
export class RsaClientPanelComponent {

    @Input()
    public client: RsaClientData = createDefaultRsaClientData("Empty");

    @Input()
    public possibleTargetClients: RsaClientData[] = [];

    @Input()
    public config: RsaConfigurationData = {
        modulusWidth: 0,
        millerRabinRounds: 0,
        randomSeed: 0,
        numberSystem: 0
    };

    @Output()
    public deleteSelf: EventEmitter<void> = new EventEmitter<void>();

    constructor(private backendRequestService: RsaBackendRequestService,
                private dialogService: DialogService) {
    }

    /**
     * Gibt an, ob bereits gesetzt wurde, an wen der Client senden soll.
     */
    public sendingToNotSet(): boolean {
        return this.client.sendingTo === undefined;
    }

    /**
     * Löscht sich selber aus der Liste der Clients.
     */
    public delete(): void {
        this.deleteSelf.emit();
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

        this.backendRequestService.encrypt(requestBody).then(response => {
            this.client.ciphertext = response.message;

            if (!this.client.keyPair) {
                this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht verschlüsselt.");
                return;
            }

            const signRequest = new RsaSignRequest(
                this.client.plaintext,
                this.client.keyPair,
                this.config.numberSystem
            );
            this.backendRequestService.sign(signRequest).then(response => {
                this.client.signature = response.message;
                this.client.signature_valid = "ungeprüft";
                this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht verschlüsselt und signiert.");
            });
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

        this.backendRequestService.decrypt(requestBody).then(response => {
            this.client.plaintext = response.message;

            if (!this.client.receivedFrom || !this.client.receivedFrom.keyPair) {
                this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht entschlüsselt.");
                return;
            }

            const verifyRequest = new RsaVerifyRequest(
                this.client.plaintext,
                this.client.signature,
                this.client.receivedFrom.keyPair,
                this.config.numberSystem
            );
            this.backendRequestService.verify(verifyRequest).then(response => {
                if (response.message === "true") {
                    this.client.signature_valid = "gültig";
                } else {
                    this.client.signature_valid = "ungültig";
                }
                this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht entschlüsselt und verifiziert.");
            });
        });
    }

    /**
     * Sendet den Ciphertext an den anderen Partner und setzt die Felder zurück.
     */
    public send(): void {
        if (!this.client.sendingTo) {
            return;
        }
        this.client.sendingTo.ciphertext = this.client.ciphertext;
        this.client.sendingTo.signature = this.client.signature;
        this.client.sendingTo.receivedFrom = this.client;

        this.clearFields();
    }

    public changeTargetClientTo(client: RsaClientData): void {
        this.client.sendingTo = client;
    }

    public clearFields(): void {
        this.client.plaintext = "";
        this.client.ciphertext = "";
        this.client.signature = "";
        this.client.signature_valid = "ungeprüft";
    }
}
