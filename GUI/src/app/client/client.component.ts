import {Component, Input} from '@angular/core';
import { CommonModule } from '@angular/common';
import {FormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";

@Component({
  selector: 'client',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatButtonModule,
    MatExpansionModule,
    MatFormFieldModule,
    MatInputModule
  ],
  templateUrl: './client.component.html',
  styleUrl: './client.component.css'
})
export class ClientComponent {

  @Input() clientName: string = "";
  @Input() public plainText: string = "";
  @Input() public cipherText: string = "";
  @Input() public signature: string = "";
  @Input() public privateExponent: string = "";
  public signatureCalculated: boolean = false;
  public signatureValid: boolean = false;

  constructor() { }

  public encrypt() {
    console.log("encrypt");
  }

  public decrypt() {
    console.log("decrypt");
  }

  public clearFields() {
    this.plainText = "";
    this.cipherText = "";
  }

  public sign() {
    console.log("sign");
  }

  public verify() {
    console.log("verify");
  }

}
