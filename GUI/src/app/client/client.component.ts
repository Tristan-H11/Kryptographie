import {Component, Input, OnInit} from '@angular/core';
import { CommonModule } from '@angular/common';
import {FormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {ClientEnum} from "../models/client-enum";
import {KeyManagementService} from "../services/key-management.service";

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
export class ClientComponent implements OnInit {

  @Input() client: ClientEnum = ClientEnum.Alice;
  public signatureCalculated: boolean = false;
  public signatureValid: boolean = false;

  public cipherText: string = "";
  public plainText: string = "";
  public signature: string = "";

  public privateExponent: string = "";
  public blockSize: string = "";

  constructor(private keyService: KeyManagementService) {

  }

  ngOnInit(): void {
    this.keyService.getKeyPair(this.client).subscribe(keyPair => {
      this.blockSize = keyPair.private_key.block_size;
      this.privateExponent = keyPair.private_key.d;
    });
  }


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
