import {Component, Input} from "@angular/core";
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
import {AbstractClientPanelComponent} from "../../shared/AbstractClientPanelComponent";
import {concatMap, EMPTY} from "rxjs";

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
})
export class MvClientPanelComponent extends AbstractClientPanelComponent<MvClientData>{

    @Input()
    public config: MvConfigurationData = {
        modulusWidth: 0,
        millerRabinRounds: 0,
        coefA: 0,
        randomSeed: 0,
        numberSystem: 0
    };

    protected readonly JSON = JSON;

    constructor(private backendRequestService: MvBackendRequestService,
                private dialogService: DialogService) {
        super();
    }

    /**
     * Verschlüsselt die Nachricht für das gewählte Ziel.
     */
    public override encrypt(): void {
        if (!this.client.sendingTo || !this.client.sendingTo.keyPair) {
            return;
        }

        let loadingCalcKey = this.dialogService.startTimedCalc();
        let request: MvEncryptRequest = {
            public_key: copyMvPublicKey(this.client.sendingTo.keyPair.public_key),
            message: this.client.plaintext,
            radix: this.config.numberSystem,
            random_seed: this.config.randomSeed
        };

        this.backendRequestService.encrypt(request).pipe(
            concatMap(ciphertext => {
                this.client.ciphertext = copyMvCipherText(ciphertext);

                if (!this.client.keyPair) {
                    this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht verschlüsselt.");
                    return EMPTY;
                }

                let body: MvSignRequest = {
                    private_key: this.client.keyPair.private_key,
                    message: this.client.plaintext,
                    random_seed: this.config.randomSeed
                };
                return this.backendRequestService.sign(body);
            })
        ).subscribe(signature => {
            this.client.signature = copyMvSignature(signature)
            this.client.signature_valid = "ungeprüft";
            this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht verschlüsselt und signiert.");
        });
    }

    /**
     * Entschlüsselt den Ciphertext und prüft die Signatur, falls vorhanden.
     */
    public override decrypt(): void {
        if (!this.client.keyPair) {
            return;
        }

        let loadingCalcKey = this.dialogService.startTimedCalc();
        let request: MvDecryptRequest = {
            private_key: copyMvKeyPair(this.client.keyPair).private_key,
            cipher_text: copyMvCipherText(this.client.ciphertext),
            radix: this.config.numberSystem
        };
        this.backendRequestService.decrypt(request).pipe(
            concatMap(plaintext => {
                this.client.plaintext = plaintext.message;

                if (!this.client.receivedFrom || !this.client.receivedFrom.keyPair) {
                    this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht entschlüsselt.");
                    return EMPTY;
                }

                let body: MvVerifyRequest = {
                    public_key: this.client.receivedFrom.keyPair.public_key,
                    message: this.client.plaintext,
                    signature: this.client.signature
                };
                this.client.receivedFrom = undefined;
                return this.backendRequestService.verify(body);
            })
        ).subscribe(result => {
            if (result.message === "true") {
                this.client.signature_valid = "gültig";
            } else {
                this.client.signature_valid = "ungültig";
            }
            this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht entschlüsselt und verifiziert.");
        });
    }

    protected createDefaultClient(name: string): MvClientData {
        return MvClientData.createDefaultWithName(name);
    }
}
