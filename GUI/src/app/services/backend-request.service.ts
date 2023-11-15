import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {EndpointsService} from "./endpoints.service";
import {firstValueFrom} from "rxjs";

@Injectable({
  providedIn: 'root'
})
export class BackendRequestService {

  constructor(
    private endpointsService: EndpointsService,
    private http: HttpClient
  ) { }

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
}
