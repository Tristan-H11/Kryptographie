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
import {ClientData} from "../models/client";
import {StateManagementService} from "../services/management/state-management.service";

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
    public modulusWidth: number = 128;
    public numberSystem: number = 55296;
    public millerRabinIterations: number = 100;
    public coefficientA: number = 5;
    public random_seed: number= 3;

    private configurationData = this.stateService.getConfigurationData();

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
        private backendRequestService: MvBackendRequestService) {
    }

    public generateKeys(client: string) {
        let config: MvKeygenConfig = {
            modulus_width: this.modulusWidth,
            miller_rabin_rounds: this.millerRabinIterations,
            coef_a: this.coefficientA,
            random_seed: this.random_seed
        };
        this.backendRequestService.createKeyPair(config).then(key => {
            if (client === "Alice") {
                this.clients[0].keyPair = copyMvKeyPair(key);
            } else {
                this.clients[1].keyPair = copyMvKeyPair(key);
            }
            console.log("Generated key pair for " + client);
            console.log(key);
            console.log(this.clients);
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
