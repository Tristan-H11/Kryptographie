import {Component, OnInit} from "@angular/core";
import {CommonModule} from "@angular/common";
import {RouterLink, RouterOutlet} from "@angular/router";
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatIconModule} from "@angular/material/icon";
import {MatDividerModule} from "@angular/material/divider";
import {MatButtonModule} from "@angular/material/button";
import {RsaBackendRequestService} from "./services/backend-api/rsa-backend-request.service";
import {MatSnackBar} from "@angular/material/snack-bar";
import {StateManagementService} from "./services/management/state-management.service";
import {MatSlideToggleModule} from "@angular/material/slide-toggle";
import {FormsModule} from "@angular/forms";
import {MatInputModule} from "@angular/material/input";
import {NavbarComponent} from "./navbar/navbar.component";

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
        NavbarComponent,
    ],
    templateUrl: "./app.component.html",
    styleUrls: ["./app.component.scss"]
})

/**
 * Main Component of the application.
 * The component is responsible for the initialization of the application and the server connection.
 * The server connection is checked at the beginning of the application.
 * The server URL can be changed in the settings.
 * The server URL is saved in the local storage.
 * The routing to Navbar and Homepage is done in the html file of app.component.html
 */
export class AppComponent implements OnInit {

    public isServerReachable: boolean = false;

    constructor(private backendRequestService: RsaBackendRequestService,
                private stateService: StateManagementService) {
    }

    /**
     * Initialize the clients and check the server connection.
     */
    ngOnInit(): void {
        this.stateService.createClient("Alice");
        this.stateService.createClient("Bob");
        this.backendRequestService.checkHealth().then((result) => {
            this.isServerReachable = result;
        });
    }
}
