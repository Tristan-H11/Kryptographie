import {Injectable} from '@angular/core';
import {ClientEnum} from "../../models/client-enum";
import {AbstractClientObservableManagementService} from "./abstract-client-observable-management-service";
import {MessageManagementService} from "./message-management.service";
import {KeyManagementService} from "./key-management.service";

@Injectable({
  providedIn: 'root'
})
/**
 * Service zum Registrieren von Clients bei allen Client-Observable-Services.
 */
export class ClientRegistrationService {

  private services: AbstractClientObservableManagementService<any>[] = [];

  constructor(
    private messageService: MessageManagementService,
    private keyService: KeyManagementService,
  ) {
    this.services.push(messageService, keyService);
  }

  /**
   * Registriert einen Client bei allen Services.
   * @param client
   * @param service
   */
  public registerClient(client: ClientEnum): void {
    this.services.forEach(service => service.registerClient(client));
  }
}
