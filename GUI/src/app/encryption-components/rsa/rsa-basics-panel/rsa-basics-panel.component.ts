import { Component } from '@angular/core';
import {MatCard, MatCardContent, MatCardHeader, MatCardTitle} from "@angular/material/card";
import {
	MatExpansionPanel,
	MatExpansionPanelDescription,
	MatExpansionPanelHeader,
	MatExpansionPanelTitle
} from "@angular/material/expansion";

@Component({
  selector: 'rsa-basics-panel',
  standalone: true,
	imports: [
		MatCard,
		MatCardContent,
		MatCardHeader,
		MatCardTitle,
		MatExpansionPanel,
		MatExpansionPanelDescription,
		MatExpansionPanelHeader,
		MatExpansionPanelTitle
	],
  templateUrl: './rsa-basics-panel.component.html',
})
export class RsaBasicsPanelComponent {

}
