import {Component} from "@angular/core";
import {
    MatAccordion,
    MatExpansionPanel,
    MatExpansionPanelActionRow,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatButton} from "@angular/material/button";
import {MatFormField, MatHint, MatLabel, MatSuffix} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {NgForOf} from "@angular/common";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {MvBackendRequestService} from "../services/backend-api/mv-backend-request.service";
import {MvKeygenConfig} from "../models/mv-keygen-config";
import {
    copyMvCipherText,
    copyMvKeyPair,
    copyMvPublicKey,
    copyMvSignature,
    MvDecryptRequest,
    MvEncryptRequest,
    MvSignRequest,
    MvVerifyRequest,
} from "../models/mv-beans";
import {createDefaultMvClientData, MvClientData} from "../models/client";
import {StateManagementService} from "../services/management/state-management.service";
import {MatCard, MatCardContent, MatCardHeader, MatCardTitle} from "@angular/material/card";
import {MvBasicsPanelComponent} from "./mv-basics-panel/mv-basics-panel.component";
import {MvClientPanelComponent} from "./mv-client-panel/mv-client-panel.component";

@Component({
    selector: "app-menezes-vanstone",
    standalone: true,
    imports: [
        MatAccordion,
        MatExpansionPanel,
        MatButton,
        MatExpansionPanelActionRow,
        MatExpansionPanelDescription,
        MatExpansionPanelHeader,
        MatExpansionPanelTitle,
        MatFormField,
        MatHint,
        MatInput,
        MatLabel,
        MatSuffix,
        NgForOf,
        ReactiveFormsModule,
        FormsModule,
        MatCard,
        MatCardTitle,
        MatCardHeader,
        MatCardContent,
        MvBasicsPanelComponent,
        MvClientPanelComponent
    ],
    templateUrl: "./menezes-vanstone.component.html",
    styleUrl: "./menezes-vanstone.component.scss"
})
/**
 * Component for the Menezes Vanstone Encryption and Decryption.
 */
export class MenezesVanstoneComponent {
    public modulusWidth: number = 128;
    public numberSystem: number = 55296;
    public millerRabinIterations: number = 100;
    public coefficientA: number = 5;
    public random_seed: { radix: number } = {radix: 55296};

    private configurationData = this.stateService.getConfigurationData();

    // TODO: Vorläufige Dummy-Implementierung. Wird noch überarbeitet
    public clients: MvClientData[] =
        [
            createDefaultMvClientData("Alice"),
            createDefaultMvClientData("Bob"),
            createDefaultMvClientData("Charlie")
        ];

    constructor(
        private stateService: StateManagementService,
        private backendRequestService: MvBackendRequestService) {
    }

    public generateKeys(client: string) {
        let config: MvKeygenConfig = {
            modulus_width: this.modulusWidth,
            miller_rabin_rounds: this.millerRabinIterations,
            coef_a: this.coefficientA,
            random_seed: this.random_seed.radix
        };
        this.backendRequestService.createKeyPair(config).then(key => {
            if (client === "Alice") {
                this.clients[0].keyPair = copyMvKeyPair(key);
            } else {
                this.clients[1].keyPair = copyMvKeyPair(key);
            }
            console.log("Generated key pair for " + client);
            console.log(key);
            console.log(this.clients);
        });
    }

    public calcMinimumBitsize(): number {
        return Math.ceil(Math.log2(this.numberSystem));
    }

    public calcMaxNumbersystem(): number {
        return 2 ** this.modulusWidth;
    }


    /**
     * Gibt eine shallow copy (!) der Clients zurück, die nicht der übergebene Client sind.
     * Damit bleiben alle Referenzen erhalten, nur der Client selbst wird nicht zurückgegeben.
     */
    public getPossibleTargetClients(client: MvClientData): MvClientData[] {
        return this.clients.filter(c => c !== client);
    }
}
