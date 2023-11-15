import {Injectable} from '@angular/core';
import {createEmptyKeyPair, KeyPair} from "../models/key-pair";
import {ClientEnum} from "../models/client-enum";
import {CreateKeyPairRequest} from "../models/create-key-pair-request";
import {BackendRequestService} from "./backend-request.service";
import {AbstractClientObservableManagementService} from './abstract-client-observable-management-service';

@Injectable({
    providedIn: 'root'
})
export class KeyManagementService extends AbstractClientObservableManagementService<KeyPair> {

    protected override createEmptyObject(): KeyPair {
        return createEmptyKeyPair();
    }

    constructor(private backendRequestService: BackendRequestService) {
        super();
    }

    public generateKeyPair(requestContent: CreateKeyPairRequest, client: ClientEnum): void {
        this.backendRequestService.createKeyPair(requestContent).then(
            (keyPair) => {
                let entry = this.clientMap.get(client);
                if (entry) {
                    entry.next(keyPair);
                } else {
                    console.log("Client " + client + " is not registered!");
                }
            }
        );
    }

    public setModul(client: ClientEnum, modulus: string): void {
        let entry = this.clientMap.get(client);
        if (entry) {
            entry.value.public_key.modulus = modulus;
        } else {
            console.log("Client " + client + " is not registered!");
        }
    }

    public getModul(client: ClientEnum) {
        let entry = this.clientMap.get(client);
        if (entry) {
            return entry.value.public_key.modulus;
        } else {
            console.log("Client " + client + " is not registered!");
            return "";
        }
    }

    public setE(client: ClientEnum, e: string): void {
        let entry = this.clientMap.get(client);
        if (entry) {
            entry.value.public_key.e = e;
        } else {
            console.log("Client " + client + " is not registered!");
        }
    }

    public getE(client: ClientEnum) {
        let entry = this.clientMap.get(client);
        if (entry) {
            return entry.value.public_key.e;
        } else {
            console.log("Client " + client + " is not registered!");
            return "";
        }
    }

    public getD(client: ClientEnum) {
        let entry = this.clientMap.get(client);
        if (entry) {
            return entry.value.private_key.d;
        } else {
            console.log("Client " + client + " is not registered!");
            return "";
        }
    }

    public setD(client: ClientEnum, d: string): void {
        let entry = this.clientMap.get(client);
        if (entry) {
            entry.value.private_key.d = d;
        } else {
            console.log("Client " + client + " is not registered!");
        }
    }
}
