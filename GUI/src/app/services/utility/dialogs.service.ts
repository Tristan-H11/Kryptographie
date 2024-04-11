import { Injectable } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';
import { MatDialog, MatDialogRef } from '@angular/material/dialog';
import { LoadingDialogComponent } from '../../loading-dialog/loading-dialog.component';

@Injectable({
    providedIn: 'root'
})
export class DialogService {

    constructor(private snackBar: MatSnackBar, private dialog: MatDialog) { }

    showSnackbar(message: string) {
        this.snackBar.open(message, "Ok", {
            duration: 5000,
        });
    }

    openLoadDialog(): MatDialogRef<LoadingDialogComponent> {
        return this.dialog.open(LoadingDialogComponent, {
            disableClose: true // Verhindert das Schließen durch den Benutzer
        });
    }
}
