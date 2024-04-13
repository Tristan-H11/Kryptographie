import {Component, Input} from "@angular/core";
import {FormsModule} from "@angular/forms";
import {MatButton} from "@angular/material/button";
import {
    MatExpansionPanel,
    MatExpansionPanelActionRow,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatFormField, MatHint, MatLabel, MatSuffix} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {NgForOf} from "@angular/common";
import {MvClientData} from "../../shared/ClientData";
import {DialogService} from "../../../services/utility/dialogs.service";
import {MvBackendRequestService} from "../../../services/backend-api/mv-backend-request.service";
import {MvKeygenConfig} from "../../../models/mv-keygen-config";
import {copyMvKeyPair} from "../../../models/mv-beans";
import {MvConfigurationData} from "../../shared/ConfigurationDataTypes";
import {
    BasicConfigurationFieldsComponent
} from "../../shared/basic-configuration-fields/basic-configuration-fields.component";

@Component({
    selector: "mv-configuration-panel",
    standalone: true,
    imports: [
        FormsModule,
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
        MatSuffix,
        NgForOf,
        BasicConfigurationFieldsComponent
    ],
    templateUrl: "./mv-configuration-panel.component.html",
    styleUrl: "./mv-configuration-panel.component.scss"
})
export class MvConfigurationPanelComponent {

    @Input()
    public config: MvConfigurationData = {
        modulusWidth: 32,
        millerRabinRounds: 20,
        coefA: 1,
        randomSeed: 3,
        numberSystem: 55296
    };

    @Input()
    public clients: MvClientData[] = [];

    constructor(private backendRequestService: MvBackendRequestService,
                private dialogService: DialogService) {
    }

    /**
     * Generiert Schlüsselpaare für den gewählten Client.
     */
    public generateKeys(name: string) {
        let loadingCalcKey = this.dialogService.startTimedCalc();
        let config: MvKeygenConfig = {
            modulus_width: this.config.modulusWidth,
            miller_rabin_rounds: this.config.millerRabinRounds,
            coef_a: this.config.coefA,
            random_seed: this.config.randomSeed
        };
        this.backendRequestService.createKeyPair(config).then(key => {
            const client = this.clients.find(client => client.name === name);
            if (client) {
                client.keyPair = copyMvKeyPair(key);
                console.log("Generated key pair for " + client);
                console.log(key);
                console.log(this.clients);
            } else {
                console.error("MV-KeypairGen: Client not found: " + name);
            }
            this.dialogService.endTimedCalc(loadingCalcKey, "Schlüsselpaar für " + name + " generiert.");
        });
    }
}
