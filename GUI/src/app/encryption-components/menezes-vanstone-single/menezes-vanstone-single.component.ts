import {Component, Input} from '@angular/core';
import {
    MatAccordion,
    MatExpansionPanel, MatExpansionPanelActionRow,
    MatExpansionPanelDescription, MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MvBasicsPanelComponent} from "../menezes-vanstone/mv-basics-panel/mv-basics-panel.component";
import {
  MvConfigurationPanelComponent
} from "../menezes-vanstone/mv-configuration-panel/mv-configuration-panel.component";
import {MvConfigurationData} from "../shared/ConfigurationDataTypes";
import {MvClientData} from "../shared/ClientData";
import {MvBackendRequestService} from "../../services/backend-api/mv-backend-request.service";
import {DialogService} from "../../services/utility/dialogs.service";
import {
    copyMvCipherText, copyMvKeyPair,
    copyMvPublicKey,
    copyMvSignature, MvDecryptRequest,
    MvEncryptRequest,
    MvSignRequest, MvVerifyRequest
} from "../../models/mv-beans";
import {concatMap, EMPTY} from "rxjs";
import {
    BasicConfigurationFieldsComponent
} from "../shared/basic-configuration-fields/basic-configuration-fields.component";
import {MatFormField, MatHint, MatLabel} from "@angular/material/form-field";
import {FormsModule} from "@angular/forms";
import {MvKeygenConfig} from "../../models/mv-keygen-config";
import {MatInput} from "@angular/material/input";
import {MatButton} from "@angular/material/button";
import {MatCard, MatCardContent, MatCardHeader, MatCardTitle} from "@angular/material/card";
import {EmptyIfUndefinedPipe} from "../../services/pipes/empty-if-undefined";

@Component({
    selector: 'app-menezes-vanstone-single',
    standalone: true,
    imports: [
        MatAccordion,
        MvBasicsPanelComponent,
        MvConfigurationPanelComponent,
        MatExpansionPanel,
        MatExpansionPanelTitle,
        MatExpansionPanelDescription,
        BasicConfigurationFieldsComponent,
        MatFormField,
        FormsModule,
        MatInput,
        MatButton,
        MatLabel,
        MatCard,
        MatCardContent,
        MatCardHeader,
        MatCardTitle,
        MatExpansionPanelActionRow,
        MatHint,
        MatExpansionPanelHeader,
        EmptyIfUndefinedPipe
    ],
    templateUrl: './menezes-vanstone-single.component.html',
    styleUrl: './menezes-vanstone-single.component.scss'
})
/**
 * Komponente für die Menezes-Vanstone-Variante des ElGamal-Verfahrens mit nur einem Schlüsselpaar.
 * Es wird also keine Kommunikation simuliert, sondern mit einem einzigen Schlüssel die Verschlüsselung und
 * Signatur dargestellt.
 */
export class MenezesVanstoneSingleComponent {
    public config: MvConfigurationData = MvConfigurationData.createDefault();
    public client: MvClientData = MvClientData.createDefaultWithName("Alice");

    protected readonly JSON = JSON;

    constructor(private backendRequestService: MvBackendRequestService,
                private dialogService: DialogService) {
    }

    /**
     * Verschlüsselt die Nachricht für das gewählte Ziel.
     */
    public encrypt(): void {
        if (!this.client || !this.client.keyPair) {
            return;
        }

        let loadingCalcKey = this.dialogService.startTimedCalc();
        let request: MvEncryptRequest = {
            public_key: copyMvPublicKey(this.client.keyPair.public_key),
            message: this.client.plaintext,
            radix: this.config.numberSystem
        };

        this.client.signature = {
            r: "",
            s: "",
            string_representation: ""
        };

        this.backendRequestService.encrypt(request).pipe(
            concatMap(ciphertext => {
                this.client.ciphertext = copyMvCipherText(ciphertext);
                this.client.plaintext = "";

                if (!this.client.keyPair) {
                    this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht verschlüsselt.");
                    return EMPTY;
                }

                let body: MvSignRequest = {
                    private_key: this.client.keyPair.private_key,
                    message: this.client.plaintext
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
        this.backendRequestService.decrypt(request).pipe(
            concatMap(plaintext => {
                this.client.plaintext = plaintext.message;

                if (!this.client || !this.client.keyPair) {
                    this.dialogService.endTimedCalc(loadingCalcKey, "Nachricht entschlüsselt.");
                    return EMPTY;
                }

                let body: MvVerifyRequest = {
                    public_key: this.client.keyPair.public_key,
                    message: this.client.plaintext,
                    signature: this.client.signature
                };
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

    public generateKeys() {
        let loadingCalcKey = this.dialogService.startTimedCalc();
        let config: MvKeygenConfig = {
            modulus_width: this.config.modulusWidth,
            miller_rabin_rounds: this.config.millerRabinRounds,
            coef_a: this.config.coefA,
            random_seed: this.config.randomSeed
        };
        this.backendRequestService.createKeyPair(config).subscribe(key => {
            if (this.client) {
                this.client.keyPair = copyMvKeyPair(key);
                console.log("Generated key pair for " + this.client);
                console.log(key);
                console.log(this.client);
            } else {
                console.error("MV-KeypairGen: Client not found: " + this.client);
            }
            this.dialogService.endTimedCalc(loadingCalcKey, "Schlüsselpaar generiert.");
        });
    }
}
