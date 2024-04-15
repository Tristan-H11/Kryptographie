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
export class AppComponent implements OnInit {

    public isServerReachable: boolean = false;

    constructor(private backendRequestService: RsaBackendRequestService) {
    }

    /**
     * Initial soll überprüft werden, ob der Server erreichbar ist.
     */
    ngOnInit(): void {
        this.backendRequestService.checkHealth().then((result) => {
            this.isServerReachable = result;
        });
    }
}
