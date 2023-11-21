import {Component, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {RouterLink, RouterOutlet} from '@angular/router';
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatIconModule} from "@angular/material/icon";
import {MatDividerModule} from "@angular/material/divider";
import {MatButtonModule} from "@angular/material/button";
import {BackendRequestService} from "./services/backend-api/backend-request.service";
import {MatSnackBar} from "@angular/material/snack-bar";
import {ClientService} from "./services/management/client.service";
import {Client} from "./models/client";

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
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  title = 'RSA-Encryption-Tool';

  constructor(private backendRequestService: BackendRequestService,
              private clientService: ClientService,
              private snackBar: MatSnackBar) {
  }

  ngOnInit(): void {
    this.registerNewClientByName("Alice");
    this.registerNewClientByName("Bob");
  }

  public registerNewClientByName(name: string) {
    this.clientService.registerServices();
    this.clientService.createAndRegisterClient(name);
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
        return this.clientService.getClients();
    }
}
