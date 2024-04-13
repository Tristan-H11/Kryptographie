import {Injectable, signal, WritableSignal} from "@angular/core";
import {Client} from "../../encryption-components/shared/IClientData";
import {RsaKeyPair} from "../../models/rsa-key-pair";
import {MessageSignatureContainer} from "../../models/message-signature-container";
import {RsaCreateKeyPairRequest} from "../../models/rsa-create-key-pair-request";

@Injectable({
    providedIn: "root"
})
export class StateManagementService {

    private use_fast_math = signal(false);

    /**
     * Gibt das Signal für die Verwendung von FastMath zurück.
     */
    public getUseFastMath(): WritableSignal<boolean> {
        return this.use_fast_math;
    }
}
