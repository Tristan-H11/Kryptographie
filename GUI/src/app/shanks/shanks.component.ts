import {Component} from "@angular/core";
import {CommonModule} from "@angular/common";
import {ShanksRoutingModule} from "./shanks-routing.module";
import {BackendRequestService} from "../services/backend-api/backend-request.service";
import {ShanksRequest} from "../models/shanks-request";
import {FormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";

@Component({
	selector: "app-shanks",
	standalone: true,
	imports: [CommonModule, ShanksRoutingModule, FormsModule, MatButtonModule, MatExpansionModule, MatFormFieldModule, MatInputModule],
	templateUrl: "./shanks.component.html",
	styleUrl: "./shanks.component.scss"
})
export class ShanksComponent {

	//Input fields
	public base = "";
	public element = "";
	public modul = "";
	//Output field
	public result = "";

	constructor(private backendRequestService: BackendRequestService) {
	}

	/**
	 * Berechne die Shanks-Operation.
	 */
	public calculate() {
		let body = new ShanksRequest(this.base, this.element, this.modul);
		this.backendRequestService.shanks(body).then(result => {
			this.result = result.message;
		});
	}
}
