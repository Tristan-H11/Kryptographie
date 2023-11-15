import {Injectable} from '@angular/core';
import {ConfigurationData, createDefaultConfigurationData} from "../../models/configuration-data";
import {BehaviorSubject} from "rxjs";

@Injectable({
    providedIn: 'root'
})
export class ConfigurationManagementService {

    private configurationDataSubject = new BehaviorSubject<ConfigurationData>(
        this.createDefaultObject()
    );

    constructor() {
    }

    protected createDefaultObject(): ConfigurationData {
        return createDefaultConfigurationData();
    }

    public setModulusWidth(modulbreite: number) {
        this.setProperty("modulus_width", modulbreite)
    }

    public getModulbreite(): number {
        return this.getProperty("modulus_width");
    }

    public setNumberSystem(zahlensystem: number) {
        this.setProperty("number_system_base", zahlensystem)
    }

    public getNumberSystem(): number {
        return this.getProperty("number_system_base");
    }

    public setRandomSeed(randomSeed: number) {
        this.setProperty("random_seed", randomSeed)
    }

    public getRandomSeed(): number {
        return this.getProperty("random_seed");
    }

    public setMillerRabinIterations(millerRabinIterations: number) {
        this.setProperty("miller_rabin_rounds", millerRabinIterations)
    }

    public getMillerRabinIterations(): number {
        return this.getProperty("miller_rabin_rounds");
    }


    /**
     * Gibt eine Property aus dem Value des BehaviorSubjects zurück, falls der Client registriert ist.
     */
    protected getProperty<K extends keyof ConfigurationData>(property: K): ConfigurationData[K] {
        return this.configurationDataSubject.value[property];
    }

    /**
     * Setzt eine Property im Value des BehaviorSubjects, falls der Client registriert ist.
     */
    protected setProperty<K extends keyof ConfigurationData>(property: K, value: ConfigurationData[K]): void {
        this.configurationDataSubject.value[property] = value;
    }

}


