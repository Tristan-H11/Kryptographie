import {Injectable} from "@angular/core";
import {StateManagementService} from "../management/state-management.service";

@Injectable({
    providedIn: "root"
})
/**
 * Service zum Abfragen der Adressen der Backend-Endpunkte.
 */
export class EndpointsService {

    private domain: string = "https://krypto-server.tristan-hoermann.de"
    private rsaEndpoint: string = "/rsa";
    private mvEndpoint: string = "/menezesVanstone";
    private mathEndpoint: string = "/math";

    /**
     * Gibt den Healthcheck-Endpoint zurück.
     */
    public getHealthcheckEndpoint(): string {
        return this.domain + "/health";
    }

    /**
     * Gibt den Endpoint zum Erstellen eines neuen MenezesVanstone Schlüsselpaares zurück.
     */
    public getMvCreateKeyPairEndpoint(): string {
        return this.domain + this.mvEndpoint + "/createKeyPair";
    }

    /**
     * Gibt den Endpoint zum MenezesVanstone Verschlüsseln einer Nachricht zurück.
     */
    public getMvEncryptEndpoint(): string {
        return this.domain + this.mvEndpoint + "/encrypt";
    }

    /**
     * Gibt den Endpoint zum MenezesVanstone Entschlüsseln einer Nachricht zurück.
     */
    public getMvDecryptEndpoint(): string {
        return this.domain + this.mvEndpoint + "/decrypt";
    }

    /**
     * Gibt den Endpoint zum MenezesVanstone Signieren einer Nachricht zurück.
     */
    public getMvSignEndpoint(): string {
        return this.domain + this.mvEndpoint + "/sign";
    }

    /**
     * Gibt den Endpoint zum MenezesVanstone Verifizieren einer Nachricht zurück.
     */
    public getMvVerifyEndpoint(): string {
        return this.domain + this.mvEndpoint + "/verify";
    }

    /**
     * Gibt den Endpoint zum Erstellen eines RSA Schlüsselpaares zurück.
     */
    public getRsaCreateKeyPairEndpoint(): string {
        return this.domain + this.rsaEndpoint + "/createKeyPair";
    }

    /**
     * Gibt den Endpoint zum RSA Verschlüsseln einer Nachricht zurück.
     */
    public getRsaEncryptEndpoint(): string {
        return this.domain + this.rsaEndpoint + "/encrypt";
    }

    /**
     * Gibt den Endpoint zum RSA Entschlüsseln einer Nachricht zurück.
     */
    public getRsaDecryptEndpoint(): string {
        return this.domain + this.rsaEndpoint + "/decrypt";
    }

    /**
     * Gibt den Endpoint zum RSA Signieren einer Nachricht zurück.
     */
    public getRsaSignEndpoint(): string {
        return this.domain + this.rsaEndpoint + "/sign";
    }

    /**
     * Gibt den Endpoint zum RSA Verifizieren einer Nachricht zurück.
     */
    public getRsaVerifyEndpoint(): string {
        return this.domain + this.rsaEndpoint + "/verify";
    }

    /**
     * Gibt den Endpoint zum Berechnen der schnellen Exponentiation zurück.
     */
    public getExponentiationEndpoint(): string {
        return this.domain + this.mathEndpoint + "/exponentiation";
    }

    /**
     * Gibt den Endpoint zum Berechnen des erweiterten Euklidischen Algorithmus zurück.
     */
    public getExtendedGcdEndpoint() {
        return this.domain + this.mathEndpoint + "/extended_euclid";
    }

    /**
     * Gibt den Endpoint zum Berechnen des Shanks-Algorithmus zurück.
     */
    public getShanksEndpoint() {
        return this.domain + this.mathEndpoint + "/shanks";
    }

    /**
     * Gibt den Endpoint zum Berechnen des modularen Inversen zurück.
     */
    getModularInverseEndpoint() {
        return this.domain + this.mathEndpoint + "/modular_inverse";
    }

    /**
     * Gibt den Endpoint zum RSA-Multiplizieren zweier Zahlen zurück.
     */
    getRsaMultiplicationEndpoint() {
        return this.domain + this.rsaEndpoint + "/multiplication";
    }
}
