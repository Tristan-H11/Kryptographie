import {Component, EventEmitter, Output} from "@angular/core";
import {MatFabButton} from "@angular/material/button";
import {MatIcon} from "@angular/material/icon";

@Component({
  selector: 'add-client-button',
  standalone: true,
	imports: [
		MatFabButton,
		MatIcon
	],
  templateUrl: './add-client-button.component.html',
  styleUrl: './add-client-button.component.scss'
})
export class AddClientButtonComponent {

    @Output()
    public addClient: EventEmitter<void> = new EventEmitter<void>();

    public pressed(): void {
        this.addClient.emit();
    }
}
