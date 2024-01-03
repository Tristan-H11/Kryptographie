import {Component} from "@angular/core";
import {CommonModule} from "@angular/common";
import {FormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {BackendRequestService} from "../services/backend-api/backend-request.service";
import {ModularInversRequest} from "../models/modular-invers-request";
import {MatDialog} from "@angular/material/dialog";
import {catchError, EMPTY} from "rxjs";
import {ErrorDialogComponent} from "../error-dialog/error-dialog.component";

@Component({
    selector: "app-modular-inverse",
    standalone: true,
    imports: [CommonModule, FormsModule, MatButtonModule, MatExpansionModule, MatFormFieldModule, MatInputModule],
    templateUrl: "./modular-inverse.component.html",
    styleUrl: "./modular-inverse.component.scss"
})
export class ModularInverseComponent {

    public n = "";
    public modul = "";
    public result = "";

    constructor(private backendRequestService: BackendRequestService, private dialog: MatDialog) {
    }

    /**
     * Berechnet des modular Inversen
     */

    public calculate() {
        let body = new ModularInversRequest(this.n, this.modul);
        this.backendRequestService.modularInverse(body).pipe(
            catchError(
                (error) => {
                    this.dialog.open(ErrorDialogComponent, {
                        data: {message: error.error.message}
                    });
                    return EMPTY;
                }
            )
        ).subscribe(result => {
            this.result = result.message;
        });
    }
}
