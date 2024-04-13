import {Component, Input} from "@angular/core";
import {
    BasicConfigurationFieldsComponent
} from "../../shared/basic-configuration-fields/basic-configuration-fields.component";
import {MatButton} from "@angular/material/button";
import {
    MatExpansionPanel,
    MatExpansionPanelActionRow,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatFormField, MatHint, MatLabel} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {NgForOf} from "@angular/common";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {RsaConfigurationData} from "../../shared/IConfigurationData";
import {RsaClientData} from "../../shared/IClientData";
import {RsaBackendRequestService} from "../../../services/backend-api/rsa-backend-request.service";
import {RsaCreateKeyPairRequest} from "../../../models/rsa-create-key-pair-request";
import {DialogService} from "../../../services/utility/dialogs.service";

@Component({
    selector: "app-rsa-configuration-panel",
    standalone: true,
    imports: [
        BasicConfigurationFieldsComponent,
        MatButton,
        MatExpansionPanel,
        MatExpansionPanelActionRow,
        MatExpansionPanelDescription,
        MatExpansionPanelHeader,
        MatExpansionPanelTitle,
        MatFormField,
        MatHint,
        MatInput,
        MatLabel,
        NgForOf,
        ReactiveFormsModule,
        FormsModule
    ],
    templateUrl: "./rsa-configuration-panel.component.html",
    styleUrl: "./rsa-configuration-panel.component.scss"
})
export class RsaConfigurationPanelComponent {

    @Input()
    public config: RsaConfigurationData = {
        modulusWidth: 1024,
        millerRabinRounds: 40,
        randomSeed: 3,
        numberSystem: 55296
    };

    @Input()
    public clients: RsaClientData[] = [];

    constructor(private rsaBackendService: RsaBackendRequestService,
                private dialogService: DialogService) {
    }

    /**
     * Erstellt ein neues RSA Schlüsselpaar für den Client.
     */
    public generateKeys(forClient: RsaClientData): void {
        let requestContent = new RsaCreateKeyPairRequest(
            this.config.modulusWidth,
            this.config.millerRabinRounds,
            this.config.randomSeed,
            this.config.numberSystem
        );

        let loadingCalcKey = this.dialogService.startTimedCalc();
        this.rsaBackendService.createKeyPair(requestContent).then(
            (keyPair) => {
                const client = this.clients.find(c => c === forClient);
                if (client) {
                    client.keyPair = keyPair;
                    this.dialogService.endTimedCalc(loadingCalcKey, "Schlüsselpaar für " + client.name + " generiert.");
                } else {
                    console.log("Client " + forClient.name + " not found.");
                }
            }
        );
    }
}
