import {ConfigurationDataTypes} from "./ConfigurationDataTypes";
import {ClientData} from "./ClientData";
import {Component, Input} from "@angular/core";
import {MatDialog} from "@angular/material/dialog";
import {SimpleDialogComponent} from "../../dialogs/simple-dialog/simple-dialog.component";

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
    public clients: CLIENT[] = [
        this.createDefaultClient("Alice"),
        this.createDefaultClient("Bob"),
    ];

    constructor(protected dialog: MatDialog) {
    }

    /**
     * Erstellt einen neuen Client mit dem übergebenen Namen.
     */
    protected abstract createDefaultClient(name: string): CLIENT;

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

    /**
     * Öffnet ein Dialogfenster, in dem der Benutzer einen Namen für einen neuen Client eingeben kann.
     * Der Client wird anschließend erstellt und der Liste hinzugefügt.
     */
    public openNameInputDialog(): void {
        const dialogRef = this.dialog.open(SimpleDialogComponent, {
            data: {name: "", aborted: false},
        });
        dialogRef.afterClosed().subscribe(result => {
            if (result.aborted) {
                return;
            }
            const newClient = this.createDefaultClient(result.name);
            this.clients.push(newClient);
        });
    }
}
