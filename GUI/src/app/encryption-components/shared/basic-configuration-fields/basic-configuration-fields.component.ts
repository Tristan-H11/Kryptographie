import {Component, Input} from "@angular/core";
import {FormsModule} from "@angular/forms";
import {MatFormField, MatHint, MatLabel, MatSuffix} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {IConfigurationData} from "../IConfigurationData";

@Component({
    selector: "basic-configuration-fields",
    standalone: true,
    imports: [
        FormsModule,
        MatFormField,
        MatHint,
        MatInput,
        MatLabel,
        MatSuffix
    ],
    templateUrl: "./basic-configuration-fields.component.html",
    styleUrl: "./basic-configuration-fields.component.scss"
})
export class BasicConfigurationFieldsComponent {

    @Input()
    public config: IConfigurationData = {
        modulusWidth: 64,
        millerRabinRounds: 40,
        randomSeed: 17,
        numberSystem: 55296
    };

    /**
     * Bestimmt die minimale Modulusbreite für eine gegebene Zahlensystemgröße.
     */
    public calcMinimumBitsize(): number {
        return Math.ceil(Math.log2(this.config.numberSystem));
    }

    /**
     * Bestimmt die maximale Zahlensystemgröße für eine gegebene Modulusbreite.
     */
    public calcMaxNumbersystem(): number {
        return 2 ** this.config.modulusWidth;
    }
}
