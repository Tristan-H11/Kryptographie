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
import {MatCard, MatCardContent, MatCardTitle} from "@angular/material/card";
import {MatList, MatListItem} from "@angular/material/list";
import {MatLine} from "@angular/material/core";
import {MatDivider} from "@angular/material/divider";
import {
    MatCell,
    MatCellDef,
    MatColumnDef,
    MatHeaderCell, MatHeaderCellDef,
    MatHeaderRow, MatHeaderRowDef,
    MatRow, MatRowDef,
    MatTable, MatTableDataSource
} from "@angular/material/table";

interface GiantStep {
    step: string;
    value: string;
}

@Component({
    selector: "app-shanks",
    standalone: true,
    imports: [CommonModule, FormsModule, MatButtonModule, MatExpansionModule, MatFormFieldModule, MatInputModule, MatCard, MatCardTitle, MatCardContent, MatList, MatListItem, MatLine, MatDivider, MatTable, MatColumnDef, MatHeaderCell, MatCell, MatHeaderRow, MatRow, MatCellDef, MatHeaderCellDef, MatHeaderRowDef, MatRowDef],
    templateUrl: "./shanks.component.html",
})
export class ShanksComponent {

    //Input fields
    public base = "";
    public element = "";
    public modul = "";
    //Output field
    public result = "";
    public dataSource = new MatTableDataSource<GiantStep>();

    displayedColumns: string[] = ['value', 'secondValue'];

    constructor(private backendRequestService: RsaBackendRequestService, private dialog: MatDialog) {
    }

    /**
     * Berechne die Shanks-Operation.
     */
    public calculate() {
        let body = new ShanksRequest(this.base, this.element, this.modul);
        this.backendRequestService.shanks(body).subscribe(result => {
            this.result = result.result;
            this.dataSource.data = result.giantsteps.map(step => ({
                step: step[0],
                value: step[1]
            }));
        });
    }
}
