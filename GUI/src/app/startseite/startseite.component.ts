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
/**
 * Komponente für die Darstellung der Startseite inklusive der Konfigurationsmöglichkeiten.
 */
export class StartseiteComponent implements OnInit {

  protected readonly ClientEnum = ClientEnum;

  constructor(private keyService: KeyManagementService,
              private configurationService: ConfigurationManagementService,
              private snackBar: MatSnackBar) {
  }

  /**
   * Generiert ein Schlüsselpaar für den Client.
   */
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

  /**
   * Registriert sich bei dem KeyService, um die öffentlichen Komponenten der Schlüssel bereitstellen zu können.
   */
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

  /**
   * Zeigt eine Snackbar mit der gegebenen Nachricht an.
   */
  private showSnackbar(message: string) {
    this.snackBar.open(message, "Ok", {
      duration: 4000,
    })
  }

  /**
   * Setzt den Modul des zu erzeugenden Schlüssels.
   */
  public set modulusWidth(value: number) {
    this.configurationService.setModulusWidth(value);
  }

  /**
   * Gibt den Modul des zu erzeugenden Schlüssels zurück.
   */
  public get modulusWidth(): number {
    return this.configurationService.getModulbreite();
  }

  /**
   * Setzt die Basis des Zahlensystems.
   */
  public set numberSystem(value: number) {
    this.configurationService.setNumberSystem(value);
  }

  /**
   * Gibt die Basis des Zahlensystems zurück.
   */
  public get numberSystem(): number {
    return this.configurationService.getNumberSystem();
  }

  /**
   * Setzt den Seed für die Zufallszahlengenerierung.
   */
  public set randomSeed(value: number) {
    this.configurationService.setRandomSeed(value);
  }

  /**
   * Gibt den Seed für die Zufallszahlengenerierung zurück.
   */
  public get randomSeed(): number {
    return this.configurationService.getRandomSeed();
  }

  /**
   * Setzt die Anzahl der Miller-Rabin-Iterationen.
   */
  public set millerRabinIterations(value: number) {
    this.configurationService.setMillerRabinIterations(value);
  }

  /**
   * Gibt die Anzahl der Miller-Rabin-Iterationen zurück.
   */
  public get millerRabinIterations(): number {
    return this.configurationService.getMillerRabinIterations();
  }

  /**
   * Setzt den Modul des Schlüssels von Alice.
   */
  public set modul_alice(modul: string) {
    this.keyService.setModul(ClientEnum.Alice, modul);
  }

  /**
   * Setzt den Modul des Schlüssels von Bob.
   */
  public set modul_bob(modul: string) {
    this.keyService.setModul(ClientEnum.Bob, modul);
  }

  /**
   * Gibt den Modul des Schlüssels von Alice zurück.
   */
  public get modul_alice(): string {
    return this.keyService.getModul(ClientEnum.Alice);
  }

  /**
   * Gibt den Modul des Schlüssels von Bob zurück.
   */
  public get modul_bob(): string {
    return this.keyService.getModul(ClientEnum.Bob);
  }

  /**
   * Setzt den öffentlichen Exponenten des Schlüssels von Alice.
   */
  public set e_alice(e: string) {
    this.keyService.setE(ClientEnum.Alice, e);
  }

  /**
   * Setzt den öffentlichen Exponenten des Schlüssels von Bob.
   */
  public set e_bob(e: string) {
    this.keyService.setE(ClientEnum.Bob, e);
  }

  /**
   * Gibt den öffentlichen Exponenten des Schlüssels von Alice zurück.
   */
  public get e_alice(): string {
    return this.keyService.getE(ClientEnum.Alice);
  }

  /**
   * Gibt den öffentlichen Exponenten des Schlüssels von Bob zurück.
   */
  public get e_bob(): string {
    return this.keyService.getE(ClientEnum.Bob);
  }
}
