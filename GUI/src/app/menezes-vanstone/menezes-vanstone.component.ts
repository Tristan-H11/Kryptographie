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
import {copyMvKeyPair,} from "../models/mv-beans";
import {createDefaultMvClientData, MvClientData} from "../models/client";
import {StateManagementService} from "../services/management/state-management.service";
import {MatCard, MatCardContent, MatCardHeader, MatCardTitle} from "@angular/material/card";
import {MvBasicsPanelComponent} from "./mv-basics-panel/mv-basics-panel.component";
import {MvClientPanelComponent} from "./mv-client-panel/mv-client-panel.component";

// TODO Auslagern
export interface MvConfiguration {
    modulusWidth: number;
    millerRabinRounds: number;
    coefA: number;
    randomSeed: number;
    numberSystem: number;
}

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
    public config: MvConfiguration = {
        modulusWidth: 32,
        millerRabinRounds: 100,
        coefA: 5,
        randomSeed: 3,
        numberSystem: 55296
    }

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
            modulus_width: this.config.modulusWidth,
            miller_rabin_rounds: this.config.millerRabinRounds,
            coef_a: this.config.coefA,
            random_seed: this.config.randomSeed
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
        return Math.ceil(Math.log2(this.config.numberSystem));
    }

    public calcMaxNumbersystem(): number {
        return 2 ** this.config.modulusWidth;
    }


    /**
     * Gibt eine shallow copy (!) der Clients zurück, die nicht der übergebene Client sind.
     * Damit bleiben alle Referenzen erhalten, nur der Client selbst wird nicht zurückgegeben.
     */
    public getPossibleTargetClients(client: MvClientData): MvClientData[] {
        return this.clients.filter(c => c !== client);
    }
}
