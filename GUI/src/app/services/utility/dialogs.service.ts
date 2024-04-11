import {Injectable} from "@angular/core";
import {MatSnackBar} from "@angular/material/snack-bar";
import {MatDialog, MatDialogRef} from "@angular/material/dialog";
import {LoadingDialogComponent} from "../../loading-dialog/loading-dialog.component";

@Injectable({
    providedIn: "root"
})
export class DialogService {
    private calculations = new Map<string, {
        startTime: number,
        loadingDialogRef: MatDialogRef<LoadingDialogComponent>
    }>();

    constructor(private snackBar: MatSnackBar, private dialog: MatDialog) {
    }

    /**
     * Startet eine Berechnung und zeigt einen Ladebildschirm an.
     * Dabei wird eine eindeutige ID zurückgegeben, die zum Beenden der Berechnung verwendet werden muss.
     * Somit ist es möglich, mehrere Berechnungen gleichzeitig zu starten und den Service als Threadsicher zu betrachten.
     */
    public startTimedCalc(): string {
        const key = Date.now().toString(); // Erzeugt einen eindeutigen Schlüssel basierend auf der aktuellen Zeit
        this.calculations.set(key, {
            startTime: Date.now(),
            loadingDialogRef: this.openLoadDialog()
        });
        return key;
    }

    /**
     * Beendet eine Berechnung und zeigt eine Snackbar mit der Dauer an.
     */
    public endTimedCalc(key: string, message: string): void {
        const calculation = this.calculations.get(key);
        if (!calculation) {
            console.error(`Keine Berechnung gefunden für den Schlüssel ${key}`);
            return;
        }
        const duration = Date.now() - calculation.startTime;
        calculation.loadingDialogRef.close();
        this.showSnackbar(`${message} Dauer: ${duration}ms`);
        this.calculations.delete(key); // Entfernt die Berechnung aus der Map, da sie abgeschlossen ist
    }

    private showSnackbar(message: string) {
        this.snackBar.open(message, "Ok", {
            duration: 5000,
        });
    }

    private openLoadDialog(): MatDialogRef<LoadingDialogComponent> {
        return this.dialog.open(LoadingDialogComponent, {
            disableClose: true // Verhindert das Schließen durch den Benutzer
        });
    }
}
