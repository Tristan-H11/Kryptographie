import {Component, Inject} from "@angular/core";
import { CommonModule } from '@angular/common';
import {MAT_DIALOG_DATA, MatDialogContent, MatDialogRef, MatDialogTitle} from "@angular/material/dialog";
import {MatProgressSpinnerModule} from "@angular/material/progress-spinner";
import {MatIconModule} from "@angular/material/icon";

@Component({
  selector: 'app-error-dialog',
  standalone: true,
    imports: [CommonModule, MatDialogContent, MatDialogTitle, MatProgressSpinnerModule, MatIconModule],
  templateUrl: './error-dialog.component.html',
  styleUrl: './error-dialog.component.scss'
})
export class ErrorDialogComponent {
    public message: string;

    constructor(
        public dialogRef: MatDialogRef<ErrorDialogComponent>,
        @Inject(MAT_DIALOG_DATA) public data: {message: string}
    ) {
        this.message = data.message;
    }
}
