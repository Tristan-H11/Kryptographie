import {Injectable, signal, WritableSignal} from "@angular/core";

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
