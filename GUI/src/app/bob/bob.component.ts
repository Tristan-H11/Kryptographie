import {Component} from '@angular/core';
import {CommonModule} from "@angular/common";
import {ClientComponent} from "../client/client.component";
import {MatInputModule} from "@angular/material/input";
import {FormsModule} from "@angular/forms";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {BobRoutingModule} from "./bob-routing.module";
import {ClientEnum} from "../models/client-enum";

@Component({
  selector: 'app-bob',
  standalone: true,
  imports: [
    CommonModule,
    ClientComponent,
    BobRoutingModule,
    MatFormFieldModule,
    MatInputModule
  ],
  templateUrl: './bob.component.html',
  styleUrl: './bob.component.css'
})
export class BobComponent {
  protected readonly ClientEnum = ClientEnum;
}
