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
import {MatSlideToggleModule} from "@angular/material/slide-toggle";
import {FormsModule} from "@angular/forms";

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
        MatSlideToggleModule,
        FormsModule,
    ],
	templateUrl: "./app.component.html",
	styleUrls: ["./app.component.scss"]
})
export class AppComponent implements OnInit {

	public isServerReachable: boolean = false;

    public get isTurboMode(): boolean {
        return this.stateService.getUseFastMath()();
    }

    public set isTurboMode(useFastMath: boolean) {
        this.stateService.getUseFastMath().update(value => useFastMath);
    }

	constructor(private backendRequestService: BackendRequestService,
				private stateService: StateManagementService,
				private snackBar: MatSnackBar) {
	}

	ngOnInit(): void {
		this.stateService.createClient("Alice");
		this.stateService.createClient("Bob");

		this.backendRequestService.checkHealth().then((result) => {
			this.isServerReachable = result;
		});
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
