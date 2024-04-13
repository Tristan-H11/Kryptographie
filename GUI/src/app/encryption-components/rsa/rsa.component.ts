import {Component} from "@angular/core";
import {AddClientButtonComponent} from "../shared/add-client-button/add-client-button.component";
import {MatAccordion} from "@angular/material/expansion";
import {MvBasicsPanelComponent} from "../menezes-vanstone/mv-basics-panel/mv-basics-panel.component";
import {MvClientPanelComponent} from "../menezes-vanstone/mv-client-panel/mv-client-panel.component";
import {
    MvConfigurationPanelComponent
} from "../menezes-vanstone/mv-configuration-panel/mv-configuration-panel.component";
import {NgForOf} from "@angular/common";
import {RsaBasicsPanelComponent} from "./rsa-basics-panel/rsa-basics-panel.component";
import {RsaConfigurationPanelComponent} from "./rsa-configuration-panel/rsa-configuration-panel.component";
import {SimpleDialogComponent} from "../../simple-dialog/simple-dialog.component";
import {createDefaultRsaClientData, RsaClientData} from "../shared/IClientData";
import {MatDialog} from "@angular/material/dialog";
import {RsaConfigurationData} from "../shared/IConfigurationData";
import {RsaClientPanelComponent} from "./rsa-client-panel/rsa-client-panel.component";

@Component({
  selector: 'app-rsa',
  standalone: true,
    imports: [
        AddClientButtonComponent,
        MatAccordion,
        MvBasicsPanelComponent,
        MvClientPanelComponent,
        MvConfigurationPanelComponent,
        NgForOf,
        RsaBasicsPanelComponent,
        RsaConfigurationPanelComponent,
        RsaClientPanelComponent
    ],
  templateUrl: './rsa.component.html',
  styleUrl: './rsa.component.scss'
})
export class RsaComponent {

    public config: RsaConfigurationData = {
        modulusWidth: 1024,
        millerRabinRounds: 40,
        randomSeed: 3,
        numberSystem: 55296
    };


    public clients: RsaClientData[] = [
        createDefaultRsaClientData("Alice"),
        createDefaultRsaClientData("Bob")
    ];

    constructor(private dialog: MatDialog) {
    }

    /**
     * Löscht den übergebenen Client aus der Liste der Clients.
     * @param client
     */
    public deleteClient(client: RsaClientData) {
        this.clients = this.clients.filter(c => c !== client);
    }

    /**
     * Gibt eine shallow copy (!) der Clients zurück, die nicht der übergebene Client sind.
     * Damit bleiben alle Referenzen erhalten, nur der Client selbst wird nicht zurückgegeben.
     */
    public getPossibleTargetClients(client: RsaClientData): RsaClientData[] {
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
            const newClient = createDefaultRsaClientData(result.name);
            this.clients.push(newClient);
        });
    }
}
