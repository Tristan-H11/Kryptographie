import {Component, OnInit} from '@angular/core';
import {MatExpansionModule} from '@angular/material/expansion';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatButtonModule} from '@angular/material/button';
import {FormsModule} from "@angular/forms";
import {StartseiteRoutingModule} from "./startseite-routing.module";
import {ClientEnum} from "../models/client-enum";
import {KeyManagementService} from "../services/key-management.service";
import {createKeyPairRequestFrom} from "../models/create-key-pair-request";
import {MatSnackBar} from "@angular/material/snack-bar";


@Component({
  selector: 'app-startseite',
  standalone: true,
  imports: [
    StartseiteRoutingModule,
    MatExpansionModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    FormsModule,
  ],
  templateUrl: './startseite.component.html',
  styleUrl: './startseite.component.css'
})
export class StartseiteComponent implements OnInit {

  public modulbreite: number = 4096;
  public zahlensystem: number = 55296;
  public random_seed: number = 13;
  public miller_rabin_iterations: number = 100;

  constructor(private keyService: KeyManagementService, private snackBar: MatSnackBar) {
  }

  private showSnackbar(message: string) {
    this.snackBar.open(message, "Ok", {
      duration: 4000,
    })
  }

  public set modul_alice(modul: string) {
    this.keyService.setModul(ClientEnum.Alice, modul);
  }

  public set modul_bob(modul: string) {
    this.keyService.setModul(ClientEnum.Bob, modul);
  }

  public get modul_alice(): string {
    return this.keyService.getModul(ClientEnum.Alice);
  }

  public get modul_bob(): string {
    return this.keyService.getModul(ClientEnum.Bob);
  }

  public set e_alice(e: string) {
    this.keyService.setE(ClientEnum.Alice, e);
  }

  public set e_bob(e: string) {
    this.keyService.setE(ClientEnum.Bob, e);
  }

  public get e_alice(): string {
    return this.keyService.getE(ClientEnum.Alice);
  }

  public get e_bob(): string {
    return this.keyService.getE(ClientEnum.Bob);
  }

  public generateKeys(client: ClientEnum) {
    let requestContent = createKeyPairRequestFrom(
      this.modulbreite,
      this.miller_rabin_iterations,
      this.random_seed,
      this.zahlensystem
    )
    this.keyService.generateKeyPair(requestContent, client);

    this.showSnackbar("Schlüsselpaar für " + ClientEnum[client] + " generiert.");
  }

  protected readonly ClientEnum = ClientEnum;

  ngOnInit(): void {
    this.keyService.getKeyPairObservableWithRegister(ClientEnum.Alice).subscribe(keyPair => {
      this.modul_alice = keyPair.public_key.modulus;
      this.e_alice = keyPair.public_key.e;
    });

    this.keyService.getKeyPairObservableWithRegister(ClientEnum.Bob).subscribe(keyPair => {
      this.modul_bob = keyPair.public_key.modulus;
      this.e_bob = keyPair.public_key.e;
    });
  }
}
