import {Component, Input, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {ClientEnum} from "../models/client-enum";
import {KeyManagementService} from "../services/key-management.service";
import {MessageManagementService} from "../services/message-management.service";
import {MatIconModule} from "@angular/material/icon";

@Component({
  selector: 'client',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatButtonModule,
    MatExpansionModule,
    MatFormFieldModule,
    MatInputModule,
    MatIconModule
  ],
  templateUrl: './client.component.html',
  styleUrl: './client.component.css'
})
export class ClientComponent implements OnInit {

  @Input() client: ClientEnum = ClientEnum.Alice;
  @Input() otherClient: ClientEnum = ClientEnum.Bob;
  public signatureCalculated: boolean = false;
  public signatureValid: boolean = false;

  public cipherText: string = "";
  public plainText: string = "";
  public signature: string = "";

  public privateExponent: string = "";
  public blockSize: string = "";

  constructor(private keyService: KeyManagementService, private messageService: MessageManagementService) {
  }

  ngOnInit(): void {
    this.keyService.getKeyPair(this.client).subscribe(keyPair => {
      this.blockSize = keyPair.private_key.block_size;
      this.privateExponent = keyPair.private_key.d;
    });

    this.messageService.getMessageOberservable(this.client).subscribe(message => {
      // Werden die Nachrichten neu gesetzt, muss die Signatur neu berechnet werden.
      this.signatureCalculated = false;
      this.cipherText = message.ciphertext;
      this.plainText = message.plaintext;
      this.signature = message.signature;
    });
  }


  public encrypt() {
    let ciphertext = this.plainText + " encrypted!"  //TODO Encrypt
    this.messageService.setCiphertext(ciphertext, this.client);
  }

  public decrypt() {
    let plaintext = this.cipherText + " decrypted!"  //TODO Decrypt
    this.messageService.setPlaintext(plaintext, this.client);
  }

  public clearFields() {
    this.messageService.setPlaintext("", this.client)
    this.messageService.setCiphertext("", this.client);
  }

  public sign() {
    let signature = this.plainText + " signed!"  //TODO Sign
    this.messageService.setSignature(signature, this.client);
  }

  public verify() {
    this.signatureCalculated = true;
    this.signatureValid = this.signatureCalculated; //TODO Verify
  }

  /**
   * Sendet die verschlüsselte Nachricht und die Signatur an den anderen Client.
   * Leert anschließend die Nachrichtenfelder.
   */
  public sendMessageAndSignature() {
    this.messageService.setCiphertext(this.cipherText, this.otherClient);
    this.messageService.setSignature(this.signature, this.otherClient);

    this.clearFields()
  }

  public clearSignatureFields() {
    this.messageService.setSignature("", this.client);
    this.signatureCalculated = false;
    this.signatureValid = false;
  }
}
