import {Component} from '@angular/core';
import {CommonModule} from "@angular/common";
import {ClientComponent} from "../client/client.component";
import {MatInputModule} from "@angular/material/input";
import {FormsModule} from "@angular/forms";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {BobRoutingModule} from "./bob-routing.module";

@Component({
  selector: 'app-bob',
  standalone: true,
  imports: [
    CommonModule,
    ClientComponent,
    BobRoutingModule,
    FormsModule,
    MatButtonModule,
    MatExpansionModule,
    MatFormFieldModule,
    MatInputModule
  ],
  templateUrl: './bob.component.html',
  styleUrl: './bob.component.css'
})
export class BobComponent {
  public name: string = "Bob"
  public cipherText: string = "";
  public plainText: string = "";
  public signature: string = "";
  public privateExponent: string = "";
}
