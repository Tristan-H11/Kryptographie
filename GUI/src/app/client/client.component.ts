import {Component, OnInit, WritableSignal} from "@angular/core";
import {CommonModule} from "@angular/common";
import {FormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {Client} from "../models/client";
import {MatIconModule} from "@angular/material/icon";
import {MatSnackBar} from "@angular/material/snack-bar";
import {BackendRequestService} from "../services/backend-api/backend-request.service";
import {ActivatedRoute} from "@angular/router";
import {MatSelectModule} from "@angular/material/select";
import {MatCardModule} from "@angular/material/card";
import {MatMenuModule} from "@angular/material/menu";
import {StateManagementService} from "../services/management/state-management.service";
import {KeyPair} from "../models/key-pair";
import {EncryptDecryptRequest} from "../models/encrypt-decrypt-request";
import {SignRequest} from "../models/sign-request";
import {VerifyRequest} from "../models/verify-request";

@Component({
	selector: "client",
	standalone: true,
	imports: [
		CommonModule,
		FormsModule,
		MatButtonModule,
		MatExpansionModule,
		MatFormFieldModule,
		MatInputModule,
		MatIconModule,
		MatSelectModule,
		MatCardModule,
		MatMenuModule
	],
	templateUrl: "./client.component.html",
	styleUrl: "./client.component.scss"
})
/**
 * Komponente für die Darstellung eines Clients.
 * Diese ist die Basis für die Darstellung von Alice, Bob und möglichen Weiteren.
 */
export class ClientComponent implements OnInit {

	protected signatureVerificationCalculated: boolean = false;
	protected signatureValid: boolean = false;
	private configurationData = this.stateService.getConfigurationData();

	/**
	 * Konstruktor der Komponente.
	 */
	constructor(private backendRequestService: BackendRequestService,
				private stateService: StateManagementService,
				private route: ActivatedRoute,
				private snackBar: MatSnackBar) {
	}

	/**
	 * Der Client, für den die Komponente dargestellt wird.
	 */
	private _client: Client | undefined;

	public get client(): Client {
		if (!this._client) {
			throw new Error("Client is undefined!");
		}
		return this._client;
	}

	private _clientKeyPair: WritableSignal<KeyPair> | undefined;

	public get clientKeyPair(): WritableSignal<KeyPair> {
		if (!this._clientKeyPair) {
			throw new Error("ClientKeyPair is undefined!");
		}
		return this._clientKeyPair;
	}

	public get sendingTo(): Client {
		if (!this.client.sendingTo) {
			throw new Error("SendingTo is undefined!");
		}
		return this.client.sendingTo;
	}

	public set sendingTo(value: Client) {
		this.client.sendingTo = value;
	}

	public get receivedFrom(): Client {
		if (!this.client.receivedFrom) {
			throw new Error("ReceivedFrom is undefined!");
		}
		return this.client.receivedFrom;
	}

	public get cipherText(): string {
		return this.stateService.getClientMessage(this.client)().ciphertext;
	}

	public set cipherText(value: string) {
		this.setCiphertext(this.client, value);
	}

	public get plainText(): string {
		return this.stateService.getClientMessage(this.client)().plaintext;
	}

	public set plainText(value: string) {
		this.setPlaintext(this.client, value);
	}

	public get signature(): string {
		return this.stateService.getClientMessage(this.client)().signature;
	}

	public set signature(value: string) {
		this.setSignature(this.client, value);
	}

	public get privateExponent(): string {
		return this.clientKeyPair().d;
	}

	public set privateExponent(d: string) {
		this.clientKeyPair.update(keyPair => ({
			...keyPair,
			d
		}));
	}

	public get modulus(): string {
		return this.clientKeyPair().modulus;
	}

	public set modulus(modulus: string) {
		this.clientKeyPair.update(keyPair => ({
			...keyPair,
			modulus
		}));
	}

	public receivedFromIsSet(): boolean {
		if (this.client.receivedFrom) {
			console.log("Received from is set: " + this.client.receivedFrom.name);
			return true;
		}
		console.log("Received from is not set");
		return false;
	}

	/**
	 * Registriert die Komponente bei den Services, um über Änderungen informiert zu werden.
	 */
	public ngOnInit(): void {
		this.route.paramMap.subscribe(params => {
			const name = params.get("client");
			console.log("OnInit in Client with name " + name);
			if (name) {
				this.initClientComponent(name);
			} else {
				console.error("Client name is null! Invalid path");
				return;
			}
		});

	}

	/**
	 * Verschlüsselt die Nachricht.
	 */
	public encrypt() {
		const requestBody = new EncryptDecryptRequest(
			this.plainText,
			this.stateService.getClientKey(this.sendingTo)(),
			this.configurationData().number_system_base
		);
		this.backendRequestService.encrypt(requestBody).then(r => {
			this.setCiphertext(this.client, r.message);
			this.showSnackbar("Nachricht verschlüsselt!");
		});
	}

	/**
	 * Entschlüsselt die Nachricht.
	 */
	public decrypt() {
		const requestBody = new EncryptDecryptRequest(
			this.cipherText,
			this.clientKeyPair(),
			this.configurationData().number_system_base
		);
		this.backendRequestService.decrypt(requestBody).then(r => {
			this.setPlaintext(this.client, r.message);
			this.showSnackbar("Nachricht entschlüsselt!");
		});
	}

	/**
	 * Berechnet die Signatur des Klartextes.
	 */
	public signPlaintext() {
		const requestBody = new SignRequest(
			this.plainText,
			this.clientKeyPair(),
		);
		this.backendRequestService.sign(requestBody).then(r => {
			this.setSignature(this.client, r.message);
			this.showSnackbar("Signatur berechnet!");
		});
	}

	/**
	 * Berechnet die Signatur des Chiffrats.
	 */
	public signCiphertext() {
		const requestBody = new SignRequest(
			this.cipherText,
			this.clientKeyPair(),
		);
		this.backendRequestService.sign(requestBody).then(r => {
			this.setSignature(this.client, r.message);
			this.showSnackbar("Signatur berechnet!");
		});
	}

	/**
	 * Verifiziert die Signatur des Klartextes.
	 */
	public verifyPlaintext() {
		const requestBody = new VerifyRequest(
			this.plainText,
			this.signature,
			this.stateService.getClientKey(this.receivedFrom)(),
		);
		this.backendRequestService.verify(requestBody).then(r => {
			let verified = r.message === "true";
			this.signatureVerificationCalculated = true;
			this.signatureValid = verified;
			this.showSnackbar("Signatur verifiziert!");
		});
	}

	/**
	 * Verifiziert die Signatur des Chiffrats.
	 */
	public verifyCiphertext() {
		const requestBody = new VerifyRequest(
			this.cipherText,
			this.signature,
			this.stateService.getClientKey(this.receivedFrom)(),
		);
		this.backendRequestService.verify(requestBody).then(r => {
			let verified = r.message === "true";
			this.signatureVerificationCalculated = true;
			this.signatureValid = verified;
			this.showSnackbar("Signatur verifiziert!");
		});
	}

	/**
	 * Sendet die verschlüsselte Nachricht und die Signatur an den anderen Client.
	 * Setzt anschließend die Nachrichten- und Signaturfelder zurück.
	 */
	public sendCiphertextAndSignature() {
		console.log("Sending message and signature from " + this.client?.name + " to " + this.sendingTo.name + "");
		this.setCiphertext(this.sendingTo, this.cipherText);
		this.setSignature(this.sendingTo, this.signature);
		this.sendingTo.receivedFrom = this.client;
		this.showSnackbar("Nachricht und Signatur gesendet!");

		// Alle Felder leeren, wenn gesendet wird
		this.clearFields();
		this.clearSignatureFields();
	}

	public sendPlaintextAndSignature() {
		console.log("Sending message and signature from " + this.client?.name + " to " + this.sendingTo.name + "");
		this.setPlaintext(this.sendingTo, this.plainText);
		this.setSignature(this.sendingTo, this.signature);
		this.sendingTo.receivedFrom = this.client;
		this.showSnackbar("Nachricht und Signatur gesendet!");

		// Alle Felder leeren, wenn gesendet wird
		this.clearFields();
		this.clearSignatureFields();
	}

	public isCiphertextEmpty(): boolean {
		return this.cipherText === "";
	}

	public isPlaintextEmpty(): boolean {
		return this.plainText === "";
	}

	/**
	 * Setzt die Nachrichtenfelder zurück.
	 */
	public clearFields() {
		this.setPlaintext(this.client, "");
		this.setCiphertext(this.client, "");
	}

	/**
	 * Setzt die Signaturfelder zurück.
	 */
	public clearSignatureFields() {
		this.setSignature(this.client, "");
		this.signatureVerificationCalculated = false;
		this.signatureValid = false;
	}

	/**
	 * Gibt alle Clients außer dem "eigenen" zurück.
	 */
	public getOtherClients(): Set<Client> {
		const allClients = this.stateService.getAllClients();
		return new Set(
			[...allClients].filter(clientFromSet => clientFromSet !== this.client)
		);
	}

	/**
	 * Gibt zurück, ob das Signaturfeld nicht leer ist.
	 */
	public signFieldIsNotEmpty() {
		return this.signature !== "";
	}

	/**
	 * Initialisiert die Komponente mit dem Client, der in der URL angegeben ist.
	 */
	private initClientComponent(name: string) {
		this._client = this.stateService.getClientByName(name);
		this.sendingTo = this.getOtherClients().values().next().value;
		console.log(this.client);
		this._clientKeyPair = this.stateService.getClientKey(this.client);
	}

	/**
	 * Zeigt eine Snackbar mit der übergebenen Nachricht an.
	 */
	private showSnackbar(message: string) {
		this.snackBar.open(message, "Ok", {
			duration: 4000,
		});
	}

	private setCiphertext(client: Client, value: string) {
		this.stateService.getClientMessage(client).update(message => ({
			...message,
			ciphertext: value
		}));
	}

	private setPlaintext(client: Client, value: string) {
		this.stateService.getClientMessage(client).update(message => ({
			...message,
			plaintext: value
		}));
	}

	private setSignature(client: Client, value: string) {
		this.stateService.getClientMessage(client).update(message => ({
			...message,
			signature: value
		}));
	}
}
