import {Component} from "@angular/core";
import {CommonModule} from "@angular/common";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {RsaBackendRequestService} from "../../services/backend-api/rsa-backend-request.service";
import {ExtendedEuclidRequest} from "../../models/extended-euclid-request";
import {catchError, EMPTY} from "rxjs";
import {ErrorDialogComponent} from "../../dialogs/error-dialog/error-dialog.component";
import {MatDialog} from "@angular/material/dialog";

@Component({
	selector: "app-extended-gcd",
	standalone: true,
	imports: [CommonModule, MatButtonModule, MatExpansionModule, MatFormFieldModule, MatInputModule, ReactiveFormsModule, FormsModule],
	templateUrl: "./extended-gcd.component.html",
})
export class ExtendedGcdComponent {
	public ggT: string = "";
	public parameterA: string = "";
	public parameterB: string = "";
	public coefficientX: string = "";
	public coefficientY: string = "";

	constructor(private backendRequestService: RsaBackendRequestService, private dialog: MatDialog) {
	}

	/**
	 * Berechnet den ggT.
	 */
	public calculate() {

		const body = new ExtendedEuclidRequest(this.parameterA, this.parameterB);

		this.backendRequestService.extendedGcd(body).pipe(
            catchError(
                (error) => {
                    this.dialog.open(ErrorDialogComponent, {
                        data: {message: error.error.message}
                    });
                    return EMPTY;
                }
            )
        ).subscribe(result => {
			this.ggT = result.ggt;
			this.coefficientX = result.x;
			this.coefficientY = result.y;
		});
	}
}
