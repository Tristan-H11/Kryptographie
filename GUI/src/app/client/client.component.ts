import {Component, Input, OnInit} from '@angular/core';
import {CommonModule} from '@angular/common';
import {FormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {ClientEnum} from "../models/client-enum";
import {KeyManagementService} from "../services/management/key-management.service";
import {MessageManagementService} from "../services/management/message-management.service";
import {MatIconModule} from "@angular/material/icon";
import {MatSnackBar} from "@angular/material/snack-bar";
import {BackendRequestService} from "../services/backend-request.service";
import {createEncryptDecryptRequestFrom} from "../models/encrypt-decrypt-request";
import {ConfigurationManagementService} from "../services/management/configuration-management.service";
import {signRequestFrom} from "../models/sign-request";
import {verifyRequestFrom} from "../models/verify-request";

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
/**
 * Komponente für die Darstellung eines Clients.
 * Diese ist die Basis für die Darstellung von Alice, Bob und möglichen Weiteren.
 */
export class ClientComponent implements OnInit {

  /**
   * Der Client, für den die Komponente dargestellt wird.
   */
  @Input() client: ClientEnum = ClientEnum.Alice;
  /**
   * Der Client, mit dem kommuniziert wird.
   */
  @Input() targetClient: ClientEnum = ClientEnum.Bob;
  public signatureVerificationCalculated: boolean = false;
  public signatureValid: boolean = false;

  /**
   * Konstruktor der Komponente.
   * @param keyService
   * @param messageService
   * @param backendRequestService
   * @param configurationService
   * @param snackBar
   */
  constructor(private keyService: KeyManagementService,
              private messageService: MessageManagementService,
              private backendRequestService: BackendRequestService,
              private configurationService: ConfigurationManagementService,
              private snackBar: MatSnackBar) {
  }

  /**
   * Registriert die Komponente bei den Services, um über Änderungen informiert zu werden.
   */
  ngOnInit(): void {
    this.keyService.getObservableWithRegister(this.client).subscribe(keyPair => {
      this.privateExponent = keyPair.d;
      this.modulus = keyPair.modulus;
    });

    this.messageService.getObservableWithRegister(this.client).subscribe(message => {
      // Werden die Nachrichten neu gesetzt, muss die Signatur neu berechnet werden.
      this.signatureVerificationCalculated = false;
      this.cipherText = message.ciphertext;
      this.plainText = message.plaintext;
      this.signature = message.signature;
    });
  }

  /**
   * Zeigt eine Snackbar mit der übergebenen Nachricht an.
   * @param message
   * @private
   */
  private showSnackbar(message: string) {
    this.snackBar.open(message, "Ok", {
      duration: 4000,
    })
  }

  /**
   * Verschlüsselt die Nachricht.
   */
  public encrypt() {
    const requestBody = createEncryptDecryptRequestFrom(
      this.plainText,
      this.keyService.getKeyPair(this.targetClient),
      this.configurationService.getNumberSystem()
    );
    this.backendRequestService.encrypt(requestBody).then(r => {
      this.messageService.setCiphertext(r.message, this.client);
      this.showSnackbar("Nachricht verschlüsselt!");
    })
  }

  /**
   * Entschlüsselt die Nachricht.
   */
  public decrypt() {
    const requestBody = createEncryptDecryptRequestFrom(
      this.cipherText,
      this.keyService.getKeyPair(this.client),
      this.configurationService.getNumberSystem()
    );
    this.backendRequestService.decrypt(requestBody).then(r => {
      this.messageService.setPlaintext(r.message, this.client);
      this.showSnackbar("Nachricht entschlüsselt!");
    })
  }

  /**
   * Berechnet die Signatur der Nachricht.
   */
  public sign() {
    const requestBody = signRequestFrom(
      this.plainText,
      this.keyService.getKeyPair(this.client),
    );
    this.backendRequestService.sign(requestBody).then(r => {
      this.messageService.setSignature(r.message, this.client);
      this.showSnackbar("Signatur berechnet!");
    })
  }

  /**
   * Verifiziert die Signatur der Nachricht.
   */
  public verify() {
    const requestBody = verifyRequestFrom(
      this.plainText,
      this.signature,
      this.keyService.getKeyPair(this.client),
    );
    this.backendRequestService.verify(requestBody).then(r => {
      let verified = r.message === "true";
      this.signatureVerificationCalculated = true;
      this.signatureValid = verified;
      this.showSnackbar("Signatur verifiziert!");
    })
  }

  /**
   * Sendet die verschlüsselte Nachricht und die Signatur an den anderen Client.
   * Setzt anschließend die Nachrichten- und Signaturfelder zurück.
   */
  public sendMessageAndSignature() {
    this.messageService.setCiphertext(this.cipherText, this.targetClient);
    this.messageService.setSignature(this.signature, this.targetClient);
    this.showSnackbar("Nachricht und Signatur gesendet!");

    // Alle Felder leeren, wenn gesendet wird
    this.clearFields();
    this.clearSignatureFields();
  }

  /**
   * Setzt die Nachrichtenfelder zurück.
   */
  public clearFields() {
    this.messageService.setPlaintext("", this.client)
    this.messageService.setCiphertext("", this.client);
  }

  /**
   * Setzt die Signaturfelder zurück.
   */
  public clearSignatureFields() {
    this.messageService.setSignature("", this.client);
    this.signatureVerificationCalculated = false;
    this.signatureValid = false;
  }

  public get cipherText(): string {
    return this.messageService.getCiphertext(this.client);
  }

  public set cipherText(value: string) {
    this.messageService.setCiphertext(value, this.client);
  }

  public get plainText(): string {
    return this.messageService.getPlaintext(this.client);
  }

  public set plainText(value: string) {
    this.messageService.setPlaintext(value, this.client);
  }

  public get signature(): string {
    return this.messageService.getSignature(this.client);
  }

  public set signature(value: string) {
    this.messageService.setSignature(value, this.client);
  }

  public get privateExponent(): string {
    return this.keyService.getD(this.client);
  }

  public set privateExponent(value: string) {
    this.keyService.setD(this.client, value);
  }

  public get modulus(): string {
    return this.keyService.getModul(this.client);
  }

  public set modulus(value: string) {
    this.keyService.setModul(this.client, value);
  }
}
