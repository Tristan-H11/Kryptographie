import {
  MAT_DIALOG_DATA,
  MatDialogActions,
  MatDialogClose,
  MatDialogContent,
  MatDialogRef,
  MatDialogTitle
} from "@angular/material/dialog";
import {Component, Inject} from "@angular/core";
import {MatButtonModule} from "@angular/material/button";
import {FormsModule} from "@angular/forms";
import {MatInputModule} from "@angular/material/input";
import {MatFormFieldModule} from "@angular/material/form-field";

@Component({
	selector: "simple-dialog",
	templateUrl: "simple-dialog.component.html",
	standalone: true,
	imports: [
		MatFormFieldModule,
		MatInputModule,
		FormsModule,
		MatButtonModule,
		MatDialogTitle,
		MatDialogContent,
		MatDialogActions,
		MatDialogClose,
	],
})
export class SimpleDialogComponent {
	constructor(
		public dialogRef: MatDialogRef<SimpleDialogComponent>,
		@Inject(MAT_DIALOG_DATA) public data: { name: string, aborted: boolean },
	) {
	}

	// TODO: Bei Escape fliegt ein Fehler, weil Result undefined ist. Result wie onNoClick handhaben.

	public onNoClick(): void {
		this.dialogRef.close(
			{name: this.data.name, aborted: true}
		);
	}

	public onEnter() {
		if (this.data.name.length < 1) {
			this.onNoClick(); // Bei leerem Namen wird das Dialogfenster abgebrochen.
			return;
		}
		this.dialogRef.close(this.data);
	}
}
