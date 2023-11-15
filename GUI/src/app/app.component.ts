import {Component, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {RouterLink, RouterOutlet} from '@angular/router';
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatIconModule} from "@angular/material/icon";
import {MatDividerModule} from "@angular/material/divider";
import {MatButtonModule} from "@angular/material/button";
import {BackendRequestService} from "./services/backend-request.service";
import {MatSnackBar} from "@angular/material/snack-bar";
import {ClientRegistrationService} from "./services/management/client-registration.service";
import {ClientEnum} from "./models/client-enum";

@Component({
  selector: 'app-root',
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
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  title = 'RSA-Encryption-Tool';

  constructor(private backendRequestService: BackendRequestService,
              private clientRegistrationService: ClientRegistrationService,
              private snackBar: MatSnackBar) {
  }

  /**
   * Registriert die Clients Alice und Bob auf allen Services.
   * Hier müssen neue initiale Clients nachgetragen werden.
   */
  ngOnInit(): void {
    this.clientRegistrationService.registerClient(ClientEnum.Alice);
    this.clientRegistrationService.registerClient(ClientEnum.Bob);
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
}
