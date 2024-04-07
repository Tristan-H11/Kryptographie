import {Component} from "@angular/core";
import {
    MatAccordion,
    MatExpansionPanel,
    MatExpansionPanelActionRow,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatButton} from "@angular/material/button";
import {MatFormField, MatHint, MatLabel, MatSuffix} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {NgForOf} from "@angular/common";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {MvBackendRequestService} from "../services/backend-api/mv-backend-request.service";
import {MvKeygenConfig} from "../models/mv-keygen-config";
import {
    copyMvCipherText,
    copyMvKeyPair,
    copyMvPublicKey,
    MvCipherText,
    MvDecryptRequest,
    MvEncryptRequest,
    MvKeyPair,
} from "../models/mv-beans";
import {Client, ClientData} from "../models/client";
import {StateManagementService} from "../services/management/state-management.service";
import {MatDialog, MatDialogRef} from "@angular/material/dialog";
import {MatSnackBar} from "@angular/material/snack-bar";
import {MvConfigurationData} from "../models/mv-configuration-data";
import {LoadingDialogComponent} from "../loading-dialog/loading-dialog.component";

@Component({
    selector: "app-menezes-vanstone",
    standalone: true,
    imports: [
        MatAccordion,
        MatExpansionPanel,
        MatButton,
        MatExpansionPanelActionRow,
        MatExpansionPanelDescription,
        MatExpansionPanelHeader,
        MatExpansionPanelTitle,
        MatFormField,
        MatHint,
        MatInput,
        MatLabel,
        MatSuffix,
        NgForOf,
        ReactiveFormsModule,
        FormsModule
    ],
    templateUrl: "./menezes-vanstone.component.html",
    styleUrl: "./menezes-vanstone.component.scss"
})
/**
 * Component for the Menezes Vanstone Encryption and Decryption.
 */
export class MenezesVanstoneComponent {
    // Value for configuration Data which is provided by the global state management service
    private configurationData = this.stateService.getConfigurationDataForMV();

    public clients: ClientData[] =
        [
            {
                name: "Alice",
                keyPair: {
                    public_key: {
                        curve: {
                            a: -25, prime: "259421157018863391010844302469063884861",
                            generator: {
                                x: "152198469913648308717544634828661961231",
                                y: "50296851635441247077790719368115682846",
                                is_infinite: false
                            },
                            order_of_subgroup: "2"
                        },
                        y: {
                            x: "26370934085012164485153092381593646122",
                            y: "126290671313284822425335475919650022666",
                            is_infinite: false
                        }
                    },
                    private_key: {
                        curve: {
                            a: -25, prime: "259421157018863391010844302469063884861",
                            generator: {
                                x: "152198469913648308717544634828661961231",
                                y: "50296851635441247077790719368115682846",
                                is_infinite: false
                            },
                            order_of_subgroup: "2"
                        },
                        x: "12401522966815986254216934185370504355"
                    }
                },
                plaintext: "",
                ciphertext: {encrypted_message: "", points: []}
            },
            {
                name: "Bob",
                keyPair: {
                    public_key: {
                        curve: {
                            a: -25, prime: "259421157018863391010844302469063884861",
                            generator: {
                                x: "152198469913648308717544634828661961231",
                                y: "50296851635441247077790719368115682846",
                                is_infinite: false
                            },
                            order_of_subgroup: "2"
                        },
                        y: {
                            x: "26370934085012164485153092381593646122",
                            y: "126290671313284822425335475919650022666",
                            is_infinite: false
                        }
                    },
                    private_key: {
                        curve: {
                            a: -25, prime: "259421157018863391010844302469063884861",
                            generator: {
                                x: "152198469913648308717544634828661961231",
                                y: "50296851635441247077790719368115682846",
                                is_infinite: false
                            },
                            order_of_subgroup: "2"
                        },
                        x: "12401522966815986254216934185370504355"
                    }
                },
                plaintext: "",
                ciphertext: {encrypted_message: "", points: []}
            }
        ];

    constructor(
        private stateService: StateManagementService,
        public dialog: MatDialog,
        private backendRequestService: MvBackendRequestService,
        private snackBar: MatSnackBar
    ) {
    }

    /**
     * Returns the modulus width for the Menezes Vanstone key pair.
     */
    public get modulusWidth(): number {
        return this.configurationData().modulus_width;
    }

    /**
     * Sets the modulus width for the Menezes Vanstone key pair.
     * @param modulus_width
     */
    public set modulusWidth(modulus_width: number) {
        this.configurationData.update(value => ({
            ...value,
            modulus_width
        }));
    }

    /**
     * Returns the number system base for the Menezes Vanstone key pair.
     */
    public get numberSystem(): number {
        return this.configurationData().numberSystem;
    }

    /**
     * Sets the number system base for the Menezes Vanstone key pair.
     * @param value
     */
    public set numberSystem(value: number) {
        this.configurationData.update(data => ({
            ...data,
            numberSystem: value
        }));
    }

    /**
     * Returns the number of Miller-Rabin iterations for the Menezes Vanstone key pair.
     */
    public get millerRabinIterations(): number {
        return this.configurationData().millerRabinIterations;
    }

    /**
     * Sets the number of Miller-Rabin iterations for the Menezes Vanstone key pair.
     * @param value
     */
    public set millerRabinIterations(value: number) {
        this.configurationData.update(data => ({
            ...data,
            millerRabinIterations: value
        }));
    }

    /**
     * Returns the coefficient A for the Menezes Vanstone key pair.
     */
    public get coefficientA(): number {
        return this.configurationData().coefficientA;
    }

    /**
     * Sets the coefficient A for the Menezes Vanstone key pair.
     * @param value
     */
    public set coefficientA(value: number) {
        this.configurationData.update(data => ({
            ...data,
            coefficientA: value
        }));
    }

    /**
     * Returns the random seed for the Menezes Vanstone key pair.
     */
    public get randomSeed(): number {
        return this.configurationData().random_seed;
    }

    /**
     * Sets the random seed for the Menezes Vanstone key pair.
     * @param value
     */
    public set randomSeed(value: number) {
        this.configurationData.update(data => ({
            ...data,
            random_seed: value
        }));
    }

    /**
     * Generates a Menezes Vanstone key pair for the given client.
     * @param client
     */
    public generateKeys(client: Client): void {
        let requestContent = new MvConfigurationData(
            this.modulusWidth,
            this.numberSystem,
            this.millerRabinIterations,
            this.coefficientA,
            this.randomSeed
        );
            this.generateKeyPair(requestContent, client);
        };

    public generateKeyPair(requestContent: MvConfigurationData, client: Client): void {
        let loadingDialog = this.openLoadDialog();
        const startTime = Date.now();
        this.backendRequestService.createKeyPair(requestContent).then(
            // todo tristan keyPair is not defined and should be defined, after that, the code should work
        //     keyPair) => {
        //         const duration = Date.now() - startTime;
        //         let entry = this.stateService.getClientKeyForMV(client);
        //         if(entry) {
        //             entry.set(keyPair);
        //         } else {
        //             console.log("Client " + client.name + " is not registered! Returning empty KeyPair and registering client.");
        //         }
        //         loadingDialog.close();
        //         this.showSnackbar("Schlüsselpaar für " + client.name + " generiert. Dauer: " + duration + "ms");
        //     }
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

    /**
     * Open a dialog to show a loading spinner.
     */
    public openLoadDialog(): MatDialogRef<LoadingDialogComponent> {
        return this.dialog.open(LoadingDialogComponent, {
            disableClose: true // Verhindert das Schließen durch den Benutzer
        });
    }

    public encrypt(name: string): void {
        let client = (name === "Alice") ? this.clients[0] : this.clients[1];
        let partner = (name === "Alice") ? this.clients[1] : this.clients[0];
        console.log("Encrypting with key: ", partner.keyPair.public_key);
        let request: MvEncryptRequest = {
            public_key: copyMvPublicKey(partner.keyPair.public_key),
            message: client.plaintext,
            radix: this.numberSystem
        };
        this.backendRequestService.encrypt(request).then(ciphertext => {
            client.ciphertext = copyMvCipherText(ciphertext);
            console.log("Encrypted message: ", JSON.stringify(ciphertext));
        });
    }

    public decrypt(name: string): void {
        let client = (name === "Alice") ? this.clients[0] : this.clients[1];
        let request: MvDecryptRequest = {
            private_key: copyMvKeyPair(client.keyPair).private_key,
            cipher_text: copyMvCipherText(client.ciphertext),
            radix: this.numberSystem
        };
        this.backendRequestService.decrypt(request).then(plaintext => {
            client.plaintext = plaintext.message;
        });
    }

    // Sendet den Ciphertext an den anderen Partner
    public send(name: string): void {
        let client = (name === "Alice") ? this.clients[0] : this.clients[1];
        let partner = (name === "Alice") ? this.clients[1] : this.clients[0];
        partner.ciphertext = copyMvCipherText(client.ciphertext);
        client.plaintext = "";
        client.ciphertext = {encrypted_message: "", points: []};
    }

    public clearFields(name: string) {
        let client = (name === "Alice") ? this.clients[0] : this.clients[1];
        client.plaintext = "";
        client.ciphertext = {encrypted_message: "", points: []};
    }

    protected readonly JSON = JSON;

    public calcMinimumBitsize(): number {
        return Math.ceil(Math.log2(this.numberSystem));
    }

    public calcMaxNumbersystem(): number {
        return 2 ** this.modulusWidth;
    }
}
