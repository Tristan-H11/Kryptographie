import {Component, OnInit} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RouterLink, RouterOutlet} from "@angular/router";
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatIconModule} from "@angular/material/icon";
import {MatDividerModule} from "@angular/material/divider";
import {MatButtonModule} from "@angular/material/button";
import {BackendRequestService} from "./services/backend-api/backend-request.service";
import {MatSnackBar} from "@angular/material/snack-bar";
import {Client} from "./models/client";
import {StateManagementService} from "./services/management/state-management.service";

@Component({
	selector: "app-root",
	standalone: true,
	imports: [
		CommonModule,
		RouterOutlet,
		RouterLink,
		MatSidenavModule,
		MatIconModule,
		MatDividerModule,
		MatButtonModule,
	],
	templateUrl: "./app.component.html",
	styleUrls: ["./app.component.scss"]
})
export class AppComponent implements OnInit {
	title = "RSA-Encryption-Tool";

	constructor(private backendRequestService: BackendRequestService,
				private stateService: StateManagementService,
				private snackBar: MatSnackBar) {
	}

	ngOnInit(): void {
		this.stateService.createClient("Alice");
		this.stateService.createClient("Bob");
	}


	/**
	 * Prüft die Erreichbarkeit des Servers.
	 */
	public checkServerConnection() {
		this.backendRequestService.checkHealth().then((result) => {
			if (result) {
				this.snackBar.open("Server is reachable!", "OK", {duration: 4000});
			} else {
				this.snackBar.open("Server is not reachable!", "OK", {duration: 4000});
			}
		});
	}

	public getClients(): Set<Client> {
		return this.stateService.getAllClients();
	}
}
