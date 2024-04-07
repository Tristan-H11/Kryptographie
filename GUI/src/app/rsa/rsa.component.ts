import {Component} from '@angular/core';
import {FormsModule} from "@angular/forms";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatButtonModule} from "@angular/material/button";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatIconModule} from "@angular/material/icon";
import {MatInputModule} from "@angular/material/input";
import {NgForOf, NgIf} from "@angular/common";
import {StateManagementService} from "../services/management/state-management.service";
import {MatDialog, MatDialogRef} from "@angular/material/dialog";
import {RsaBackendRequestService} from "../services/backend-api/rsa-backend-request.service";
import {MatSnackBar} from "@angular/material/snack-bar";
import {Client} from "../models/client";
import {RsaConfigurationData} from "../models/rsa-configuration-data";
import {LoadingDialogComponent} from "../loading-dialog/loading-dialog.component";
import {SimpleDialogComponent} from "../simple-dialog/simple-dialog.component";

@Component({
    selector: 'app-rsa',
    standalone: true,
    imports: [
        MatExpansionModule,
        MatFormFieldModule,
        MatInputModule,
        MatButtonModule,
        FormsModule,
        NgForOf,
        MatIconModule,
        NgIf,
    ],
    templateUrl: './rsa.component.html',
    styleUrl: './rsa.component.scss'
})
/**
 * Component for the RSA Encryption and Decryption.
 */
export class RsaComponent {
    // Value for configuration Data which is provided by the global state management service
    private configurationData = this.stateService.getConfigurationDataForRSA();

    constructor(
        private stateService: StateManagementService,
        public dialog: MatDialog,
        private backendRequestService: RsaBackendRequestService,
        private snackBar: MatSnackBar
    ) {
    }

    /**
     * Returns the modulus width for the RSA key pair.
     */
    public get modulusWidth(): number {
        return this.configurationData().modulus_width;
    }

    /**
     * Sets the modulus width for the RSA key pair.
     * @param modulus_width
     */
    public set modulusWidth(modulus_width: number) {
        this.configurationData.update(value => ({
            ...value,
            modulus_width
        }));
    }

    /**
     * Returns the number system base for the RSA key pair.
     */
    public get numberSystem(): number {
        return this.configurationData().number_system_base;
    }

    /**
     * Sets the number system base for the RSA key pair.
     * @param value
     */
    public set numberSystem(value: number) {
        this.configurationData.update(data => ({
            ...data,
            number_system_base: value
        }));
    }

    /**
     * Returns the random seed for the RSA key pair.
     */
    public get randomSeed(): number {
        return this.configurationData().random_seed;
    }

    /**
     * Sets the random seed for the RSA key pair.
     * @param value
     */
    public set randomSeed(value: number) {
        this.configurationData.update(data => ({
            ...data,
            random_seed: value
        }));
    }

    /**
     * Returns the number of Miller-Rabin iterations for the RSA key pair.
     */
    public get millerRabinIterations(): number {
        return this.configurationData().miller_rabin_rounds;
    }

    /**
     * Sets the number of Miller-Rabin iterations for the RSA key pair.
     * @param value
     */
    public set millerRabinIterations(value: number) {
        this.configurationData.update(data => ({
            ...data,
            miller_rabin_rounds: value
        }));
    }

    /**
     * Generates a new RSA key pair for the given client.
     */
    public generateKeys(client: Client) {
        let requestContent = new RsaConfigurationData(
            this.modulusWidth,
            this.millerRabinIterations,
            this.randomSeed,
            this.numberSystem
        );
        this.generateKeyPair(requestContent, client);
    }

    /**
     * Open a dialog to show a loading spinner.
     */
    public openLoadDialog(): MatDialogRef<LoadingDialogComponent> {
        return this.dialog.open(LoadingDialogComponent, {
            disableClose: true // Verhindert das Schließen durch den Benutzer
        });
    }

    /**
     * Returns the modulus of the RSA key pair for the given client.
     * @param client
     */
    public getModulus(client: Client): string {
        const keyPairSignal = this.stateService.getClientKeyForRSA(client);
        return keyPairSignal().modulus || "";
    }

    /**
     * Sets the modulus of the RSA key pair for the given client.
     * @param client
     * @param modulus
     */
    public setModulus(client: Client, modulus: string): void {
        const keyPairSignal = this.stateService.getClientKeyForRSA(client);
        keyPairSignal.update(keyPair => ({
            ...keyPair,
            modulus
        }));
    }

    /**
     * Returns the exponent of the RSA key pair for the given client.
     * @param client
     */
    public getExponent(client: Client): string {
        const keyPairSignal = this.stateService.getClientKeyForRSA(client);
        return keyPairSignal().e || "";
    }

    /**
     * Sets the exponent of the RSA key pair for the given client.
     * @param client
     * @param value
     */
    public setExponent(client: Client, value: string): void {
        const keyPair = this.stateService.getClientKeyForRSA(client);
        keyPair.update(keyPair => ({
            ...keyPair,
            e: value
        }));
    }

    /**
     * Returns the block size for the RSA key pair for the given client.
     * @param client
     */
    public getBlockSizePub(client: Client): string {
        const keyPairSignal = this.stateService.getClientKeyForRSA(client);
        return keyPairSignal().block_size_pub || "";
    }

    /**
     * Returns the block size for the RSA key pair for the given client.
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
            }
        };
    }

    /**
     * Returns all clients.
     */
    public getClients(): Set<Client> {
        return this.stateService.getAllClients();
    }

    /**
     * Deletes the given client.
     */
    public deleteClient(client: Client) {
        this.stateService.deleteClient(client);
    }

    /**
     * Generates a new RSA key pair for the given client.
     */
    private generateKeyPair(requestContent: RsaConfigurationData, client: Client): void {
        let loadingDialog = this.openLoadDialog();
        const startTime = Date.now();
        this.backendRequestService.createKeyPair(requestContent).then(
            (keyPair) => {
                const duration = Date.now() - startTime;
                let entry = this.stateService.getClientKeyForRSA(client);
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
     * Shows a snackbar with the given message.
     */
    private showSnackbar(message: string) {
        this.snackBar.open(message, "Ok", {
            duration: 5000,
        });
    }
}
