import {Component, EventEmitter, Input, Output} from "@angular/core";
import {MatFormField, MatLabel} from "@angular/material/form-field";
import {FormsModule} from "@angular/forms";
import {MatCard, MatCardContent, MatCardHeader, MatCardTitle} from "@angular/material/card";
import {
    MatExpansionPanel,
    MatExpansionPanelActionRow,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatInput} from "@angular/material/input";
import {MvClientData} from "../../shared/ClientData";
import {NgForOf} from "@angular/common";
import {MatOption, MatSelect} from "@angular/material/select";
import {MatButton} from "@angular/material/button";
import {
    copyMvCipherText,
    copyMvKeyPair,
    copyMvPublicKey,
    copyMvSignature,
    MvDecryptRequest,
    MvEncryptRequest,
    MvSignRequest,
    MvVerifyRequest
} from "../../../models/mv-beans";
import {MvBackendRequestService} from "../../../services/backend-api/mv-backend-request.service";
import {EmptyIfUndefinedPipe} from "../../../services/pipes/empty-if-undefined";
import {DialogService} from "../../../services/utility/dialogs.service";
import {MatAutocomplete, MatAutocompleteTrigger} from "@angular/material/autocomplete";
import {MatChip, MatChipListbox, MatChipOption} from "@angular/material/chips";
import {ClientActionRowComponent} from "../../shared/client-action-row/client-action-row.component";
import {MvConfigurationData} from "../../shared/ConfigurationDataTypes";

@Component({
    selector: "mv-client-panel",
    standalone: true,
    imports: [
        MatFormField,
        FormsModule,
        MatCard,
        MatCardHeader,
        MatCardTitle,
        MatExpansionPanel,
        MatExpansionPanelHeader,
        MatExpansionPanelTitle,
        MatExpansionPanelActionRow,
        MatCardContent,
        MatInput,
        MatLabel,
        MatExpansionPanelDescription,
        NgForOf,
        MatSelect,
        MatOption,
        MatButton,
        EmptyIfUndefinedPipe,
        MatAutocompleteTrigger,
        MatAutocomplete,
        MatChipListbox,
        MatChip,
        MatChipOption,
        ClientActionRowComponent,
    ],
    templateUrl: "./mv-client-panel.component.html",
    styleUrl: "./mv-client-panel.component.scss"
})
export class MvClientPanelComponent {

    @Input()
    public client: MvClientData = MvClientData.createDefaultWithName("Empty");

    @Input()
    public possibleTargetClients: MvClientData[] = [];

    @Input()
    public config: MvConfigurationData = {
        modulusWidth: 0,
        millerRabinRounds: 0,
        coefA: 0,
        randomSeed: 0,
        numberSystem: 0
    };

    @Output()
    public deleteSelf: EventEmitter<void> = new EventEmitter<void>();

    protected readonly JSON = JSON;

    constructor(private backendRequestService: MvBackendRequestService,
                private dialogService: DialogService) {
    }

    /**
     * Verschlüsselt die Nachricht für das gewählte Ziel.
     */
    public encrypt(): void {
        if (!this.client.sendingTo || !this.client.sendingTo.keyPair) {
            return;
        }

        let loadingCalcKey = this.dialogService.startTimedCalc();
        let request: MvEncryptRequest = {
            public_key: copyMvPublicKey(this.client.sendingTo.keyPair.public_key),
            message: this.client.plaintext,
            radix: this.config.numberSystem
        };
        // TODO Refactor! Verschachtelte Request sind ein NO-GO!
        this.backendRequestService.encrypt(request).then(ciphertext => {
            this.client.ciphertext = copyMvCipherText(ciphertext);

            if (!this.client.keyPair) {
                this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht verschlüsselt.");
                return;
            }

            let body: MvSignRequest = {
                private_key: this.client.keyPair.private_key,
                message: this.client.plaintext
            };
            this.backendRequestService.sign(body).then(signature => {
                this.client.signature = signature;
                this.client.signature_valid = "ungeprüft";
                this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht verschlüsselt.");
            });
        });
    }

    /**
     * Entschlüsselt den Ciphertext und prüft die Signatur, falls vorhanden.
     */
    public decrypt(): void {
        if (!this.client.keyPair) {
            return;
        }

        let loadingCalcKey = this.dialogService.startTimedCalc();
        let request: MvDecryptRequest = {
            private_key: copyMvKeyPair(this.client.keyPair).private_key,
            cipher_text: copyMvCipherText(this.client.ciphertext),
            radix: this.config.numberSystem
        };
        this.backendRequestService.decrypt(request).then(plaintext => {
            this.client.plaintext = plaintext.message;

            if (!this.client.receivedFrom || !this.client.receivedFrom.keyPair) {
                this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht entschlüsselt.");
                return;
            }
            let body: MvVerifyRequest = {
                public_key: this.client.receivedFrom.keyPair.public_key,
                message: this.client.plaintext,
                signature: this.client.signature
            };
            this.backendRequestService.verify(body).then(result => {
                if (result.message === "true") {
                    this.client.signature_valid = "gültig";
                } else {
                    this.client.signature_valid = "ungültig";
                }
                this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht entschlüsselt.");
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
        this.client.sendingTo.ciphertext = copyMvCipherText(this.client.ciphertext);
        this.client.sendingTo.signature = copyMvSignature(this.client.signature);
        this.client.sendingTo.receivedFrom = this.client

        this.clearFields();
    }

    /**
     * Setzt alle Felder des Clients zurück.
     */
    public clearFields(): void {
        this.client.plaintext = "";
        this.client.ciphertext.encrypted_message = "";
        this.client.ciphertext.points = [];
        this.client.signature.r = "Empty";
        this.client.signature.s = "Empty";
        this.client.signature_valid = "ungeprüft";
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

    public changeTargetClientTo(client: MvClientData): void {
        this.client.sendingTo = client;
    }
}
