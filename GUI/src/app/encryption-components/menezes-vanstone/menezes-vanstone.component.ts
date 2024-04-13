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
import {MatIcon} from "@angular/material/icon";
import {MvClientData} from "../shared/ClientData";
import {AddClientButtonComponent} from "../shared/add-client-button/add-client-button.component";
import {MvConfigurationData} from "../shared/ConfigurationDataTypes";
import {AbstractAsymEncryptionComponent} from "../shared/AbstractAsymEncryptionComponent";
import {SimpleDialogComponent} from "../../dialogs/simple-dialog/simple-dialog.component";


@Component({
    selector: "menezes-vanstone",
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
export class MenezesVanstoneComponent extends AbstractAsymEncryptionComponent<MvConfigurationData, MvClientData>{

    public config: MvConfigurationData = MvConfigurationData.createDefault();

    public clients: MvClientData[] =
        [
            MvClientData.createDefaultWithName("Alice"),
            MvClientData.createDefaultWithName("Bob"),
        ];

    public openNameInputDialog(): void {
        const dialogRef = this.dialog.open(SimpleDialogComponent, {
            data: {name: "", aborted: false},
        });
        dialogRef.afterClosed().subscribe(result => {
            if (result.aborted) {
                return;
            }
            const newClient = MvClientData.createDefaultWithName(result.name);
            this.clients.push(newClient);
        });
    }
}
