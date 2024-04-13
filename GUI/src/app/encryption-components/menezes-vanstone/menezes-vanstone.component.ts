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
import {MatCard, MatCardContent, MatCardHeader, MatCardTitle} from "@angular/material/card";
import {MvBasicsPanelComponent} from "./mv-basics-panel/mv-basics-panel.component";
import {MvClientPanelComponent} from "./mv-client-panel/mv-client-panel.component";
import {MvConfigurationPanelComponent} from "./mv-configuration-panel/mv-configuration-panel.component";
import {SimpleDialogComponent} from "../../dialogs/simple-dialog/simple-dialog.component";
import {MatDialog} from "@angular/material/dialog";
import {MatIcon} from "@angular/material/icon";
import {createDefaultMvClientData, MvClientData} from "../shared/IClientData";
import {AddClientButtonComponent} from "../shared/add-client-button/add-client-button.component";
import {MvConfigurationData} from "../shared/IConfigurationData";


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
        MatIcon,
        AddClientButtonComponent
    ],
    templateUrl: "./menezes-vanstone.component.html",
    styleUrl: "./menezes-vanstone.component.scss"
})
/**
 * Component for the Menezes Vanstone Encryption and Decryption.
 */
export class MenezesVanstoneComponent {
    public config: MvConfigurationData = {
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
