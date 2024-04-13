import {ConfigurationDataTypes} from "./ConfigurationDataTypes";
import {ClientData} from "./ClientData";
import {Component, Injectable, Input} from "@angular/core";
import {MatDialog} from "@angular/material/dialog";

/**
 * Abstrakte Komponente für asymmetrische Verschlüsselung.
 * Hier werden die gemeinsamen Attribute und Methoden der Komponenten für asymmetrische Verschlüsselung definiert.
 */
@Component({
    selector: "abstract-asym-encryption",
    template: ""
})
export abstract class AbstractAsymEncryptionComponent<CONFIG extends ConfigurationDataTypes, CLIENT extends ClientData> {

    @Input()
    public abstract config: CONFIG;

    @Input()
    public abstract clients: CLIENT[];

    constructor(protected dialog: MatDialog) {
    }

    /**
     * Löscht den übergebenen Client aus der Liste der Clients.
     * @param client
     */
    public deleteClient(client: CLIENT) {
        this.clients = this.clients.filter(c => c !== client);
    }

    /**
     * Gibt eine shallow copy (!) der Clients zurück, die nicht der übergebene Client sind.
     * Damit bleiben alle Referenzen erhalten, nur der Client selbst wird nicht zurückgegeben.
     */
    public getPossibleTargetClients(client: CLIENT): CLIENT[] {
        return this.clients.filter(c => c !== client);
    }


}
