import {Component} from "@angular/core";
import {CommonModule} from "@angular/common";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {BackendRequestService} from "../services/backend-api/backend-request.service";
import {MatButtonModule} from "@angular/material/button";
import {ExponentiationRequest} from "../models/exponentiation-request";
import {catchError, EMPTY} from "rxjs";
import {ErrorDialogComponent} from "../error-dialog/error-dialog.component";
import {MatDialog} from "@angular/material/dialog";

@Component({
	selector: "app-exponentiation",
	standalone: true,
	imports: [CommonModule, MatExpansionModule, MatFormFieldModule, MatInputModule, ReactiveFormsModule, FormsModule, MatButtonModule],
	templateUrl: "./exponentiation.component.html",
	styleUrl: "./exponentiation.component.scss"
})
export class ExponentiationComponent {

	public exponent = "";
	public base = "";
	public modulus = "";
	public result = "";

	constructor(private backendRequestService: BackendRequestService, private dialog: MatDialog) {
	}

	/**
	 * Berechnet die Exponentiation.
	 */
	public calculate() {
		let body = new ExponentiationRequest(this.exponent, this.base, this.modulus);
		this.backendRequestService.exponentiation(body).pipe(
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
