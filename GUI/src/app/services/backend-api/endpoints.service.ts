import {Injectable} from "@angular/core";

@Injectable({
    providedIn: "root"
})
/**
 * Service zum Abfragen der Adressen der Backend-Endpunkte.
 */
export class EndpointsService {

    private domain: string = "http://localhost:8080";
    private rsaEndpoint: string = "/rsa";
    private mathEndpoint: string = "/math";

    constructor() {
    }

    /**
     * Gibt den Healthcheck-Endpoint zurück.
     */
    public getHealthcheckEndpoint(): string {
        return this.domain + "/health";
    }

    /**
     * Gibt den Endpoint zum Erstellen eines Schlüsselpaares zurück.
     */
    public getCreateKeyPairEndpoint(): string {
        return this.domain + this.rsaEndpoint + "/createKeyPair";
    }

    /**
     * Gibt den Endpoint zum Verschlüsseln einer Nachricht zurück.
     */
    public getEncryptEndpoint(): string {
        return this.domain + this.rsaEndpoint + "/encrypt";
    }

    /**
     * Gibt den Endpoint zum Entschlüsseln einer Nachricht zurück.
     */
    public getDecryptEndpoint(): string {
        return this.domain + this.rsaEndpoint + "/decrypt";
    }

    /**
     * Gibt den Endpoint zum Signieren einer Nachricht zurück.
     */
    public getSignEndpoint(): string {
        return this.domain + this.rsaEndpoint + "/sign";
    }

    /**
     * Gibt den Endpoint zum Verifizieren einer Nachricht zurück.
     */
    public getVerifyEndpoint(): string {
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
}
