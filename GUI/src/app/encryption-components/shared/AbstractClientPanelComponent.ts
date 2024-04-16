import {ClientData, MvClientData, RsaClientData} from "./ClientData";
import {Component, EventEmitter, Input} from "@angular/core";

/**
 * Abstrakte Komponente für Client-Panel.
 */
@Component({
    selector: "abstract-client-panel",
    template: ""
})
export abstract class AbstractClientPanelComponent<CLIENT extends ClientData> {

    @Input()
    public client: CLIENT = this.createDefaultClient("Empty");

    @Input()
    public possibleTargetClients: CLIENT[] = [];

    @Input()
    public deleteSelf: EventEmitter<void> = new EventEmitter<void>();

    /**
     * Verschlüsselt die Nachricht für das gewählte Ziel.
     */
    protected abstract encrypt(): void;

    /**
     * Entschlüsselt die Nachricht für das gewählte Ziel.
     */
    protected abstract decrypt(): void;

    /**
     * Erstellt einen neuen Client mit dem übergebenen Namen.
     */
    protected abstract createDefaultClient(name: string): CLIENT;

    /**
     * Gibt an, ob bereits gesetzt wurde, an wen der Client senden soll.
     */
    public sendingToNotSet(): boolean {
        return this.client.sendingTo === undefined;
    }

    /**
     * Löscht sich selber aus der Liste der Clients.
     */
    public delete(): void {
        this.deleteSelf.emit();
    }

    /**
     * Sendet den Ciphertext an den anderen Partner und setzt die Felder zurück.
     */
    public send(): void {
        if (!this.client.sendingTo) {
            return;
        }
        this.client.sendingTo.ciphertext = this.client.ciphertext;
        this.client.sendingTo.signature = this.client.signature;
        this.client.sendingTo.receivedFrom = this.client;

        this.clearFields();
    }

    /**
     * Setzt das Ziel, an das der Client senden soll.
     */
    public changeTargetClientTo(client: CLIENT): void {
        this.client.sendingTo = client;
    }

    /**
     * Setzt alle Felder des Clients zurück.
     */
    public clearFields(): void {
        this.client.plaintext = "";
        this.client.ciphertext = "";
        this.client.signature = "";
        this.client.signature_valid = "ungeprüft";
    }
}
