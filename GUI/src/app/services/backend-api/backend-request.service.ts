import {Injectable} from "@angular/core";
import {HttpClient} from "@angular/common/http";
import {EndpointsService} from "./endpoints.service";
import {firstValueFrom} from "rxjs";
import {ConfigurationData} from "../../models/configuration-data";
import {KeyPair} from "../../models/key-pair";
import {EncryptDecryptRequest} from "../../models/encrypt-decrypt-request";
import {SingleMessageModel} from "../../models/SingleMessageModel";
import {SignRequest} from "../../models/sign-request";
import {VerifyRequest} from "../../models/verify-request";
import {ExponentiationRequest} from "../../models/exponentiation-request";
import {ExtendedEuclidRequest} from "../../models/extended-euclid-request";
import {ExtendedEuclidResponse} from "../../models/extended-euclid-response";

@Injectable({
	providedIn: "root"
})
/**
 * Service zum Abfragen der Backend-Endpunkte.
 */
export class BackendRequestService {

	constructor(
		private endpointsService: EndpointsService,
		private http: HttpClient
	) {
	}

	/**
	 * Fragt den Healthcheck-Endpoint ab und gibt zurück, ob der Server erreichbar ist.
	 */
	public async checkHealth(): Promise<boolean> {
		try {
			await firstValueFrom(
				this.http.get(this.endpointsService.getHealthcheckEndpoint())
			);
			return true;
		} catch {
			return false;
		}
	}

	/**
	 * Fragt den Post Endpunkt zum Ertellen eines neuen Schlüsselpaares ab.
	 */
	public async createKeyPair(body: ConfigurationData): Promise<KeyPair> {
		return firstValueFrom(
			this.http.post<KeyPair>(this.endpointsService.getCreateKeyPairEndpoint(), body)
		);
	}

	/**
	 * Fragt den Post Endpunkt zum Verschlüsseln einer Nachricht ab.
	 */
	public async encrypt(body: EncryptDecryptRequest): Promise<SingleMessageModel> {
		return firstValueFrom(
			this.http.post<SingleMessageModel>(this.endpointsService.getEncryptEndpoint(), body)
		);
	}

	/**
	 * Fragt den Post Endpunkt zum Entschlüsseln einer Nachricht ab.
	 */
	public async decrypt(body: EncryptDecryptRequest): Promise<SingleMessageModel> {
		return firstValueFrom(
			this.http.post<SingleMessageModel>(this.endpointsService.getDecryptEndpoint(), body)
		);
	}

	/**
	 * Fragt den Post Endpunkt zum Signieren einer Nachricht ab.
	 */
	public async sign(body: SignRequest): Promise<SingleMessageModel> {
		return firstValueFrom(
			this.http.post<SingleMessageModel>(this.endpointsService.getSignEndpoint(), body)
		);
	}

	/**
	 * Fragt den Post Endpunkt zum Verifizieren einer Nachricht ab.
	 */
	public async verify(body: VerifyRequest): Promise<SingleMessageModel> {
		return firstValueFrom(
			this.http.post<SingleMessageModel>(this.endpointsService.getVerifyEndpoint(), body)
		);
	}

	public async exponentiation(body: ExponentiationRequest): Promise<SingleMessageModel> {
		return firstValueFrom(
			this.http.post<SingleMessageModel>(this.endpointsService.getExponentiationEndpoint(), body)
		);
	}

	public async extendedGcd(body: ExtendedEuclidRequest): Promise<ExtendedEuclidResponse> {
		return firstValueFrom(
			this.http.post<ExtendedEuclidResponse>(this.endpointsService.getExtendedGcdEndpoint(), body)
		);
	}
}
