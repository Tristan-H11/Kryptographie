import {Component} from '@angular/core';
import {RouterLink} from '@angular/router';
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatIconModule} from "@angular/material/icon";
import {MatDividerModule} from "@angular/material/divider";
import {MatButtonModule} from "@angular/material/button";
import {StateManagementService} from "../services/management/state-management.service";
import {Client} from "../encryption-components/shared/client";
import {NgForOf, NgIf} from "@angular/common";

@Component({
    selector: 'app-navbar',
    standalone: true,
    imports: [
        RouterLink,
        MatSidenavModule,
        MatIconModule,
        MatDividerModule,
        MatButtonModule,
        NgForOf,
        NgIf
    ],
    templateUrl: './navbar.component.html',
    styleUrls: ['./navbar.component.scss']
})

/**
 * Component for the navigation bar.
 * This component is used to navigate between different pages.
 * It also provides the option to enable or disable the turbo mode.
 */
export class NavbarComponent {
    constructor(private stateService: StateManagementService) {
    }

    public get isTurboMode(): boolean {
        return this.stateService.getUseFastMath()();
    }

    public set isTurboMode(useFastMath: boolean) {
        this.stateService.getUseFastMath().update(value => useFastMath);
    }

    /**
     * Schaltet den Turbo-Modus um.
     */
    public toggleTurboMode() {
        this.stateService.getUseFastMath().update(value => !value);
    }

    public getClients(): Set<Client> {
        return this.stateService.getAllClients();
    }
}
