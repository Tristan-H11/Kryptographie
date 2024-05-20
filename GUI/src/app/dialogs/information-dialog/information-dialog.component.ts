import {Component, Inject} from '@angular/core';
import {MAT_DIALOG_DATA, MatDialogContent, MatDialogRef, MatDialogTitle} from "@angular/material/dialog";
import {MatIconModule} from "@angular/material/icon";
import {CommonModule} from "@angular/common";
import {MatProgressSpinnerModule} from "@angular/material/progress-spinner";

@Component({
  selector: 'app-information-dialog',
  standalone: true,
    imports: [CommonModule, MatDialogContent, MatDialogTitle, MatProgressSpinnerModule, MatIconModule],
  templateUrl: './information-dialog.component.html',
  styleUrl: './information-dialog.component.scss'
})
export class InformationDialogComponent {
    public message: string;

    constructor(
        public dialogRef: MatDialogRef<InformationDialogComponent>,
        @Inject(MAT_DIALOG_DATA) public data: {message: string}
    ) {
        this.message = data.message;
    }
}
