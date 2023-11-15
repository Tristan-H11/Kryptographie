import {Component, OnInit} from '@angular/core';
import {MatExpansionModule} from '@angular/material/expansion';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatButtonModule} from '@angular/material/button';
import {FormsModule} from "@angular/forms";
import {StartseiteRoutingModule} from "./startseite-routing.module";
import {ClientEnum} from "../models/client-enum";
import {KeyManagementService} from "../services/management/key-management.service";
import {createConfigurationDataFrom} from "../models/configuration-data";
import {MatSnackBar} from "@angular/material/snack-bar";
import {ConfigurationManagementService} from "../services/management/configuration-management.service";


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

  protected readonly ClientEnum = ClientEnum;

  constructor(private keyService: KeyManagementService,
              private configurationService: ConfigurationManagementService,
              private snackBar: MatSnackBar) {
  }

  public generateKeys(client: ClientEnum) {
    let requestContent = createConfigurationDataFrom(
        this.modulusWidth,
        this.millerRabinIterations,
        this.randomSeed,
        this.numberSystem
    )
    this.keyService.generateKeyPair(requestContent, client);

    this.showSnackbar("Schlüsselpaar für " + ClientEnum[client] + " generiert.");
  }

  ngOnInit(): void {
    this.keyService.getObservableWithRegister(ClientEnum.Alice).subscribe(keyPair => {
      this.modul_alice = keyPair.modulus;
      this.e_alice = keyPair.e;
    });

    this.keyService.getObservableWithRegister(ClientEnum.Bob).subscribe(keyPair => {
      this.modul_bob = keyPair.modulus;
      this.e_bob = keyPair.e;
    });
  }

  private showSnackbar(message: string) {
    this.snackBar.open(message, "Ok", {
      duration: 4000,
    })
  }

  public set modulusWidth(value: number) {
    this.configurationService.setModulusWidth(value);
  }

  public get modulusWidth(): number {
    return this.configurationService.getModulbreite();
  }

  public set numberSystem(value: number) {
    this.configurationService.setNumberSystem(value);
  }

  public get numberSystem(): number {
    return this.configurationService.getNumberSystem();
  }

  public set randomSeed(value: number) {
    this.configurationService.setRandomSeed(value);
  }

  public get randomSeed(): number {
    return this.configurationService.getRandomSeed();
  }

  public set millerRabinIterations(value: number) {
    this.configurationService.setMillerRabinIterations(value);
  }

  public get millerRabinIterations(): number {
    return this.configurationService.getMillerRabinIterations();
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
}
