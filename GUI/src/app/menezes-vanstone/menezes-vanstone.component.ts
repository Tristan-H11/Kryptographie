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
    copyMvPublicKey, copyMvSignature,
    MvDecryptRequest,
    MvEncryptRequest,
    MvSignRequest,
    MvVerifyRequest,
} from "../models/mv-beans";
import {ClientData} from "../models/client";
import {StateManagementService} from "../services/management/state-management.service";
import {MatCard, MatCardContent, MatCardHeader, MatCardTitle} from "@angular/material/card";

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
        FormsModule,
        MatCard,
        MatCardTitle,
        MatCardHeader,
        MatCardContent
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
    public random_seed: number = 3;

    private configurationData = this.stateService.getConfigurationData();

    // TODO: Vorläufige Dummy-Implementierung. Wird noch überarbeitet
    public clients: ClientData[] =
        [
            {
                name: "Alice",
                keyPair: {
                    public_key: {
                        curve: {
                            a: NaN, prime: "Empty",
                            generator: {
                                x: "Empty",
                                y: "Empty",
                                is_infinite: false
                            },
                            order_of_subgroup: "Empty"
                        },
                        y: {
                            x: "Empty",
                            y: "Empty",
                            is_infinite: false
                        }
                    },
                    private_key: {
                        curve: {
                            a: NaN, prime: "Empty",
                            generator: {
                                x: "Empty",
                                y: "Empty",
                                is_infinite: false
                            },
                            order_of_subgroup: "Empty"
                        },
                        x: "Empty"
                    }
                },
                plaintext: "",
                ciphertext: {encrypted_message: "", points: []},
                signature: {r: "Empty", s: "Empty"},
                signature_valid: "ungeprüft"
            },
            {
                name: "Bob",
                keyPair: {
                    public_key: {
                        curve: {
                            a: NaN, prime: "Empty",
                            generator: {
                                x: "Empty",
                                y: "Empty",
                                is_infinite: false
                            },
                            order_of_subgroup: "Empty"
                        },
                        y: {
                            x: "Empty",
                            y: "Empty",
                            is_infinite: false
                        }
                    },
                    private_key: {
                        curve: {
                            a: NaN, prime: "Empty",
                            generator: {
                                x: "Empty",
                                y: "Empty",
                                is_infinite: false
                            },
                            order_of_subgroup: "Empty"
                        },
                        x: "Empty"
                    }
                },
                plaintext: "",
                ciphertext: {encrypted_message: "", points: []},
                signature: {r: "Empty", s: "Empty"},
                signature_valid: "ungeprüft"
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
        let request: MvEncryptRequest = {
            public_key: copyMvPublicKey(partner.keyPair.public_key),
            message: client.plaintext,
            radix: this.numberSystem
        };
        // TODO Refactor! Verschachtelte Request sind ein NO-GO!
        this.backendRequestService.encrypt(request).then(ciphertext => {
            client.ciphertext = copyMvCipherText(ciphertext);

            let body: MvSignRequest = {
                private_key: client.keyPair.private_key,
                message: client.plaintext
            };
            this.backendRequestService.sign(body).then(signature => {
                client.signature = signature;
                client.signature_valid = "ungeprüft";
            });
        });
    }

    public decrypt(name: string): void {
        let client = (name === "Alice") ? this.clients[0] : this.clients[1];
        let partner = (name === "Alice") ? this.clients[1] : this.clients[0];

        let request: MvDecryptRequest = {
            private_key: copyMvKeyPair(client.keyPair).private_key,
            cipher_text: copyMvCipherText(client.ciphertext),
            radix: this.numberSystem
        };
        this.backendRequestService.decrypt(request).then(plaintext => {
            client.plaintext = plaintext.message;

            let body: MvVerifyRequest = {
                public_key: partner.keyPair.public_key,
                message: client.plaintext,
                signature: client.signature
            };
            this.backendRequestService.verify(body).then(result => {
                if (result.message === "true") {
                    client.signature_valid = "gültig";
                } else {
                    client.signature_valid = "ungültig";
                }
            });
        });
    }

    // Sendet den Ciphertext an den anderen Partner
    public send(name: string): void {
        let client = (name === "Alice") ? this.clients[0] : this.clients[1];
        let partner = (name === "Alice") ? this.clients[1] : this.clients[0];
        partner.ciphertext = copyMvCipherText(client.ciphertext);
        partner.signature = copyMvSignature(client.signature);

        client.signature_valid = "ungeprüft";
        client.signature = {r: "Empty", s: "Empty"};
        client.plaintext = "";
        client.ciphertext = {encrypted_message: "", points: []};
    }

    public clearFields(name: string) {
        let client = (name === "Alice") ? this.clients[0] : this.clients[1];
        client.plaintext = "";
        client.ciphertext = {encrypted_message: "", points: []};
        client.signature = {r: "Empty", s: "Empty"};
        client.signature_valid = "ungeprüft";
    }

    protected readonly JSON = JSON;

    public calcMinimumBitsize(): number {
        return Math.ceil(Math.log2(this.numberSystem));
    }

    public calcMaxNumbersystem(): number {
        return 2 ** this.modulusWidth;
    }
}
