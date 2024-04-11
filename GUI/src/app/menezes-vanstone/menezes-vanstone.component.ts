import {Component} from "@angular/core";
import {
    MatAccordion,
    MatExpansionPanel,
    MatExpansionPanelActionRow,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatButton, MatFabButton} from "@angular/material/button";
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
import {MvConfigurationPanelComponent} from "./mv-configuration-panel/mv-configuration-panel.component";
import {SimpleDialogComponent} from "../simple-dialog/simple-dialog.component";
import {MatDialog, MatDialogRef} from "@angular/material/dialog";
import {MatIcon} from "@angular/material/icon";

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
        MvClientPanelComponent,
        MvConfigurationPanelComponent,
        MatFabButton,
        MatIcon
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
        ];

    constructor(public dialog: MatDialog) {
    }

    /**
     * Löscht den übergebenen Client aus der Liste der Clients.
     * @param client
     */
    public deleteClient(client: MvClientData) {
        this.clients = this.clients.filter(c => c !== client);
    }

    /**
     * Gibt eine shallow copy (!) der Clients zurück, die nicht der übergebene Client sind.
     * Damit bleiben alle Referenzen erhalten, nur der Client selbst wird nicht zurückgegeben.
     */
    public getPossibleTargetClients(client: MvClientData): MvClientData[] {
        return this.clients.filter(c => c !== client);
    }

    public openNameInputDialog(): void {
        const dialogRef = this.dialog.open(SimpleDialogComponent, {
            data: {name: "", aborted: false},
        });
        dialogRef.afterClosed().subscribe(result => {
            if (result.aborted) {
                return;
            }
            const newClient = createDefaultMvClientData(result.name);
            this.clients.push(newClient);
        });
    }
}
