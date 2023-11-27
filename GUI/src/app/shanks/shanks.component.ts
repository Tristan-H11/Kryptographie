import {Component} from "@angular/core";
import {CommonModule} from "@angular/common";
import {ShanksRoutingModule} from "./shanks-routing.module";

@Component({
	selector: "app-shanks",
	standalone: true,
	imports: [CommonModule, ShanksRoutingModule],
	templateUrl: "./shanks.component.html",
	styleUrl: "./shanks.component.scss"
})
export class ShanksComponent {

}
