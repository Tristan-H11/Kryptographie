import {Injectable} from '@angular/core';
import {ConfigurationData, createDefaultConfigurationData} from "../../models/configuration-data";
import {BehaviorSubject} from "rxjs";

@Injectable({
  providedIn: 'root'
})
/**
 * Service zum Verwalten der allgemeinen RSA-Konfiguration.
 */
export class ConfigurationManagementService {

  private configurationDataSubject = new BehaviorSubject<ConfigurationData>(
    this.createDefaultObject()
  );

  constructor() {
  }

  /**
   * Gibt ein Default-Objekt vom Typ ConfigurationData zurück.
   */
  protected createDefaultObject(): ConfigurationData {
    return createDefaultConfigurationData();
  }

  /**
   * Setzt die Breite des Moduls.
   */
  public setModulusWidth(modulbreite: number) {
    this.setProperty("modulus_width", modulbreite)
  }

  /**
   * Gibt die Breite des Moduls zurück.
   */
  public getModulbreite(): number {
    return this.getProperty("modulus_width");
  }

  /**
   * Setzt die Basis des Zahlensystems.
   */
  public setNumberSystem(zahlensystem: number) {
    this.setProperty("number_system_base", zahlensystem)
  }

  /**
   * Gibt die Basis des Zahlensystems zurück.
   */
  public getNumberSystem(): number {
    return this.getProperty("number_system_base");
  }

  /**
   * Setzt den Seed für die Zufallszahlengenerierung.
   */
  public setRandomSeed(randomSeed: number) {
    this.setProperty("random_seed", randomSeed)
  }

  /**
   * Gibt den Seed für die Zufallszahlengenerierung zurück.
   */
  public getRandomSeed(): number {
    return this.getProperty("random_seed");
  }

  /**
   * Setzt die Anzahl der Miller-Rabin-Iterationen.
   */
  public setMillerRabinIterations(millerRabinIterations: number) {
    this.setProperty("miller_rabin_rounds", millerRabinIterations)
  }

  /**
   * Gibt die Anzahl der Miller-Rabin-Iterationen zurück.
   */
  public getMillerRabinIterations(): number {
    return this.getProperty("miller_rabin_rounds");
  }

  /**
   * Gibt eine Property aus dem Value des BehaviorSubjects zurück, falls der Client registriert ist.
   */
  private getProperty<K extends keyof ConfigurationData>(property: K): ConfigurationData[K] {
    return this.configurationDataSubject.value[property];
  }

  /**
   * Setzt eine Property im Value des BehaviorSubjects, falls der Client registriert ist.
   */
  private setProperty<K extends keyof ConfigurationData>(property: K, value: ConfigurationData[K]): void {
    this.configurationDataSubject.value[property] = value;
  }
}
