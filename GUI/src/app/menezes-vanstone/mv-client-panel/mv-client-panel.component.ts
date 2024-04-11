import {Component, Input} from "@angular/core";
import {MatFormField, MatLabel} from "@angular/material/form-field";
import {FormsModule} from "@angular/forms";
import {MatCard, MatCardContent, MatCardHeader, MatCardTitle} from "@angular/material/card";
import {
    MatExpansionPanel, MatExpansionPanelActionRow,
    MatExpansionPanelDescription,
    MatExpansionPanelHeader,
    MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatInput} from "@angular/material/input";
import {createDefaultMvClientData, MvClientData} from "../../models/client";
import {NgForOf} from "@angular/common";
import {MatOption, MatSelect} from "@angular/material/select";
import {MatButton} from "@angular/material/button";
import {
    copyMvCipherText,
    copyMvKeyPair,
    copyMvPublicKey,
    copyMvSignature,
    MvDecryptRequest,
    MvEncryptRequest,
    MvSignRequest,
    MvVerifyRequest
} from "../../models/mv-beans";
import {MvBackendRequestService} from "../../services/backend-api/mv-backend-request.service";

@Component({
    selector: "mv-client-panel",
    standalone: true,
    imports: [
        MatFormField,
        FormsModule,
        MatCard,
        MatCardHeader,
        MatCardTitle,
        MatExpansionPanel,
        MatExpansionPanelHeader,
        MatExpansionPanelTitle,
        MatExpansionPanelActionRow,
        MatCardContent,
        MatInput,
        MatLabel,
        MatExpansionPanelDescription,
        NgForOf,
        MatSelect,
        MatOption,
        MatButton,
    ],
    templateUrl: "./mv-client-panel.component.html",
    styleUrl: "./mv-client-panel.component.scss"
})
export class MvClientPanelComponent {

    @Input()
    public client: MvClientData = createDefaultMvClientData("Empty");

    @Input()
    public possibleTargetClients: MvClientData[] = [];

    // Dieser Input ist ein Workaround, weil Objekte per @Input() als Referenz übergeben werden und damit immer
    // der aktuelle Wert vorliegt.
    @Input()
    public numberSystem: { radix: number } = {radix: 55296};

    protected readonly JSON = JSON;

    constructor(private backendRequestService: MvBackendRequestService,) {

    }

    /**
     * Verschlüsselt die Nachricht für das gewählte Ziel.
     */
    public encrypt(): void {
        if (!this.client.sendingTo) {
            return;
        }

        let request: MvEncryptRequest = {
            public_key: copyMvPublicKey(this.client.sendingTo.keyPair.public_key),
            message: this.client.plaintext,
            radix: this.numberSystem.radix
        };
        // TODO Refactor! Verschachtelte Request sind ein NO-GO!
        this.backendRequestService.encrypt(request).then(ciphertext => {
            this.client.ciphertext = copyMvCipherText(ciphertext);

            let body: MvSignRequest = {
                private_key: this.client.keyPair.private_key,
                message: this.client.plaintext
            };
            this.backendRequestService.sign(body).then(signature => {
                this.client.signature = signature;
                this.client.signature_valid = "ungeprüft";
            });
        });

    }

    /**
     * Entschlüsselt den Ciphertext und prüft die Signatur, falls vorhanden.
     */
    public decrypt(): void {
        let request: MvDecryptRequest = {
            private_key: copyMvKeyPair(this.client.keyPair).private_key,
            cipher_text: copyMvCipherText(this.client.ciphertext),
            radix: this.numberSystem.radix
        };
        this.backendRequestService.decrypt(request).then(plaintext => {
            this.client.plaintext = plaintext.message;

            if (!this.client.receivedFrom) {
                return;
            }
            let body: MvVerifyRequest = {
                public_key: this.client.receivedFrom.keyPair.public_key,
                message: this.client.plaintext,
                signature: this.client.signature
            };
            this.backendRequestService.verify(body).then(result => {
                if (result.message === "true") {
                    this.client.signature_valid = "gültig";
                } else {
                    this.client.signature_valid = "ungültig";
                }
            });
        });
    }

    /**
     * Sendet den Ciphertext an den anderen Partner und setzt die Felder zurück.
     */
    public send(): void {
        if (!this.client.sendingTo) {
            return;
        }
        this.client.sendingTo.ciphertext = copyMvCipherText(this.client.ciphertext);
        this.client.sendingTo.signature = copyMvSignature(this.client.signature);
        this.client.receivedFrom = this.client

        this.clearFields();
    }

    /**
     * Setzt alle Felder des Clients zurück.
     */
    public clearFields(): void {
        this.client.plaintext = "";
        this.client.ciphertext.encrypted_message = "";
        this.client.ciphertext.points = [];
        this.client.signature.r = "Empty";
        this.client.signature.s = "Empty";
        this.client.signature_valid = "ungeprüft";
    }

    /**
     * Gibt an, ob der ausgewählte Partner bereits ein Schlüsselpaar generiert hat, falls bereits ein Partner
     * ausgwählt wurde.
     * TODO: Ist noch ziemlich unschön.
     */
    public partnerHasNoKeyPairSet(): boolean {
        if (this.client.sendingTo) {
            return this.client.sendingTo.keyPair.public_key.curve.prime === "Empty";
        }
        return true;
    }

    /**
     * Gibt an, ob bereits gesetzt wurde, an wen der Client senden soll.
     */
    public sendingToNotSet(): boolean {
        return this.client.sendingTo === undefined;
    }
}
