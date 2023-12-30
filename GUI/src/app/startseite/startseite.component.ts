import {Component} from "@angular/core";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatInputModule} from "@angular/material/input";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatButtonModule} from "@angular/material/button";
import {FormsModule} from "@angular/forms";
import {Client} from "../models/client";
import {ConfigurationData} from "../models/configuration-data";
import {MatSnackBar} from "@angular/material/snack-bar";
import {NgForOf} from "@angular/common";
import {MatIconModule} from "@angular/material/icon";
import {MatDialog, MatDialogRef} from "@angular/material/dialog";
import {SimpleDialogComponent} from "../simple-dialog/simple-dialog.component";
import {LoadingDialogComponent} from "../loading-dialog/loading-dialog.component";
import {StateManagementService} from "../services/management/state-management.service";
import {BackendRequestService} from "../services/backend-api/backend-request.service";


@Component({
    selector: "app-startseite",
    standalone: true,
    imports: [
        MatExpansionModule,
        MatFormFieldModule,
        MatInputModule,
        MatButtonModule,
        FormsModule,
        NgForOf,
        MatIconModule,
    ],
    templateUrl: "./startseite.component.html",
    styleUrl: "./startseite.component.scss"
})
/**
 * Komponente für die Darstellung der Startseite inklusive der Konfigurationsmöglichkeiten.
 */
export class StartseiteComponent {

    private configurationData = this.stateService.getConfigurationData();

    constructor(private stateService: StateManagementService,
                public dialog: MatDialog,
                private backendRequestService: BackendRequestService,
                private snackBar: MatSnackBar) {
    }

    public get modulusWidth(): number {
        return this.configurationData().modulus_width;
    }

    public set modulusWidth(modulus_width: number) {
        this.configurationData.update(value => ({
            ...value,
            modulus_width
        }));
    }

    public get numberSystem(): number {
        return this.configurationData().number_system_base;
    }

    public set numberSystem(value: number) {
        this.configurationData.update(data => ({
            ...data,
            number_system_base: value
        }));
    }

    public get randomSeed(): number {
        return this.configurationData().random_seed;
    }

    public set randomSeed(value: number) {
        this.configurationData.update(data => ({
            ...data,
            random_seed: value
        }));
    }

    public get millerRabinIterations(): number {
        return this.configurationData().miller_rabin_rounds;
    }

    public set millerRabinIterations(value: number) {
        this.configurationData.update(data => ({
            ...data,
            miller_rabin_rounds: value
        }));
    }

    /**
     * Generiert ein Schlüsselpaar für den Client.
     */
    public generateKeys(client: Client) {
        let requestContent = new ConfigurationData(
            this.modulusWidth,
            this.millerRabinIterations,
            this.randomSeed,
            this.numberSystem
        );
        this.generateKeyPair(requestContent, client);
    }

    /**
     * Öffnet den Laden-Dialog.
     */
    public openLoadDialog(): MatDialogRef<LoadingDialogComponent> {
        return this.dialog.open(LoadingDialogComponent, {
            disableClose: true // Verhindert das Schließen durch den Benutzer
        });
    }

    public getModulus(client: Client): string {
        const keyPairSignal = this.stateService.getClientKey(client);
        return keyPairSignal().modulus || "";
    }

    public setModulus(client: Client, modulus: string): void {
        const keyPairSignal = this.stateService.getClientKey(client);
        keyPairSignal.update(keyPair => ({
            ...keyPair,
            modulus
        }));
    }

    public getExponent(client: Client): string {
        const keyPairSignal = this.stateService.getClientKey(client);
        return keyPairSignal().e || "";
    }

    public setExponent(client: Client, value: string): void {
        const keyPair = this.stateService.getClientKey(client);
        keyPair.update(keyPair => ({
            ...keyPair,
            e: value
        }));
    }

    /**
     * Gibt den BindingContext für die Schlüsselverwaltung dynamischer Clients zurück.
     */
    public getBindingContext(client: Client) {
        const component = this;
        return {
            get modulus(): string {
                return component.getModulus(client);
            },
            set modulus(value) {
                component.setModulus(client, value);
            },
            get exponent(): string {
                return component.getExponent(client);
            },
            set exponent(value) {
                component.setExponent(client, value);
            },
        };
    }

    /**
     * Öffnet einen Dialog, um einen neuen Client zu erstellen.
     */
    public openNameInputDialog(): void {
        const dialogRef = this.dialog.open(SimpleDialogComponent, {
            data: {name: "", aborted: false},
        });

        dialogRef.afterClosed().subscribe(result => {
            if (result.aborted) {
                return;
            }
            this.stateService.createClient(result.name);
        });
    }

    public getClients(): Set<Client> {
        return this.stateService.getAllClients();
    }

    /**
     * Löscht einen Client und entfernt alle Registrierungen.
     */
    public deleteClient(client: Client) {
        this.stateService.deleteClient(client);
    }

    /**
     * Generiert ein Schlüsselpaar mit der gegebenen Konfiguration für den Client.
     */
    private generateKeyPair(requestContent: ConfigurationData, client: Client): void {
        let loadingDialog = this.openLoadDialog();
        const startTime = Date.now();
        this.backendRequestService.createKeyPair(requestContent).then(
            (keyPair) => {
                const duration = Date.now() - startTime;
                let entry = this.stateService.getClientKey(client);
                if (entry) {
                    entry.set(keyPair);
                } else {
                    console.log("Client " + client + " is not registered!");
                }
                loadingDialog.close();
                this.showSnackbar("Schlüsselpaar für " + client.name + " generiert. Dauer: " + duration + "ms");
            }
        );
    }

    /**
     * Zeigt eine Snackbar mit der gegebenen Nachricht an.
     */
    private showSnackbar(message: string) {
        this.snackBar.open(message, "Ok", {
            duration: 5000,
        });
    }
}
