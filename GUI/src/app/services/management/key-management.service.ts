import {Injectable} from '@angular/core';
import {createEmptyKeyPair, KeyPair} from "../../models/key-pair";
import {ClientEnum} from "../../models/client-enum";
import {ConfigurationData} from "../../models/configuration-data";
import {BackendRequestService} from "../backend-request.service";
import {AbstractClientObservableManagementService} from './abstract-client-observable-management-service';

@Injectable({
    providedIn: 'root'
})
export class KeyManagementService extends AbstractClientObservableManagementService<KeyPair>{

    protected override createDefaultObject(): KeyPair {
        return createEmptyKeyPair();
    }

    constructor(private backendRequestService: BackendRequestService) {
        super();
    }

    public generateKeyPair(requestContent: ConfigurationData, client: ClientEnum): void {
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

    public getKeyPair(client: ClientEnum): KeyPair {
        return this.getValue(client);
    }

    public setModul(client: ClientEnum, modulus: string): void {
        this.setProperty(client, "modulus", modulus)
    }

    public getModul(client: ClientEnum): string {
        return this.getPropertyAsString(client, "modulus");
    }

    public setE(client: ClientEnum, e: string): void {
        this.setProperty(client, "e", e);
    }

    public getE(client: ClientEnum): string {
        return this.getPropertyAsString(client, "e");
    }

    public setBlockSizePub(client: ClientEnum, blockSize: string): void {
        this.setProperty(client, "block_size_pub", blockSize);
    }

    public getBlockSizePub(client: ClientEnum): string {
        return this.getPropertyAsString(client, "block_size_pub");
    }

    public setBlockSizePriv(client: ClientEnum, blockSize: string): void {
        this.setProperty(client, "block_size_priv", blockSize);
    }

    public getBlockSizePriv(client: ClientEnum): string {
        return this.getPropertyAsString(client, "block_size_priv");
    }

    public setD(client: ClientEnum, d: string): void {
        this.setProperty(client, "d", d);
    }

    public getD(client: ClientEnum): string {
        return this.getPropertyAsString(client, "d");
    }
}
