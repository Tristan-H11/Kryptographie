import {Component, OnInit, signal} from "@angular/core";
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
import {MatInputModule} from "@angular/material/input";

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
        MatInputModule,
    ],
	templateUrl: "./app.component.html",
	styleUrls: ["./app.component.scss"]
})
export class AppComponent implements OnInit {

	public isServerReachable: boolean = false;
	public serverURL: string = this.stateService.getServerUrl()();

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
     * Schaltet den Turbo-Modus um.
     */
    public toggleTurboMode() {
        this.isTurboMode = !this.isTurboMode;
    }

	/**
	 * Prüft die Erreichbarkeit des Servers.
	 */
	public checkServerConnection() {
		this.backendRequestService.checkHealth().then((result) => {
			if (result) {
				this.snackBar.open("Server ist erreichbar!", "OK", {duration: 4000});
			} else {
				this.snackBar.open("Server ist nicht erreichbar!", "OK", {duration: 4000});
			}
		});
	}

	public getClients(): Set<Client> {
		return this.stateService.getAllClients();
	}

	public saveServerURL() {
		this.stateService.setServerUrl(this.serverURL);
		this.snackBar.open("Server URL gespeichert!", "OK", {duration: 4000});
		setTimeout(() => {
			this.checkServerConnection();
		}, 4000);
	}
}
