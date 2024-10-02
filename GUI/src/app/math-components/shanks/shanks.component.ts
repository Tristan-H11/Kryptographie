import {Component} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RsaBackendRequestService} from "../../services/backend-api/rsa-backend-request.service";
import {ShanksRequest} from "../../models/shanks-request";
import {FormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {MatDialog} from "@angular/material/dialog";
import {
    MatCell,
    MatCellDef,
    MatColumnDef,
    MatHeaderCell,
    MatHeaderCellDef,
    MatHeaderRow,
    MatHeaderRowDef,
    MatRow,
    MatRowDef,
    MatTable,
    MatTableDataSource
} from "@angular/material/table";

interface StepValuePair {
    step: string;
    value: string;
}

@Component({
    selector: "app-shanks",
    standalone: true,
    imports: [CommonModule, FormsModule, MatButtonModule, MatExpansionModule, MatFormFieldModule, MatInputModule, MatTable, MatColumnDef, MatHeaderCell, MatCell, MatHeaderRow, MatRow, MatCellDef, MatHeaderCellDef, MatHeaderRowDef, MatRowDef],
    templateUrl: "./shanks.component.html",
})
export class ShanksComponent {

    //Input fields
    public base = "";
    public element = "";
    public modul = "";
    //Output field
    public result = "";
    public giantStepsDataSource = new MatTableDataSource<StepValuePair>();
    public babyStepsDataSource = new MatTableDataSource<StepValuePair>();

    displayedColumns: string[] = ["value", "secondValue"];

    constructor(private backendRequestService: RsaBackendRequestService, private dialog: MatDialog) {
    }

    /**
     * Berechne die Shanks-Operation.
     */
    public calculate() {
        let body = new ShanksRequest(this.base, this.element, this.modul);
        this.backendRequestService.shanks(body).subscribe(result => {
            this.result = result.result;
            this.giantStepsDataSource.data = result.giantsteps.map(step => ({
                step: step[0],
                value: step[1]
            }));
            this.babyStepsDataSource.data = result.babysteps.map(step => ({
                step: step[0],
                value: step[1]
            }));
        });
    }

    public shouldHighlightRow(row: StepValuePair): boolean {
        console.log(this.babyStepsDataSource?.data.at(-1)?.value)
        console.log(row.value);
        return row.value == this.babyStepsDataSource?.data.at(-1)?.value;
    }
}
