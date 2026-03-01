import {Component} from "@angular/core";
import {CommonModule} from "@angular/common";
import {FormsModule} from "@angular/forms";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {MatButtonModule} from "@angular/material/button";
import {GghBackendRequestService, GghCreateKeyPairRequest} from "../../services/backend-api/ggh-backend-request.service";
import {GghKeyPairBean} from "../../models/ggh-beans";

@Component({
    selector: "app-ggh",
    standalone: true,
    imports: [
        CommonModule,
        FormsModule,
        MatExpansionModule,
        MatFormFieldModule,
        MatInputModule,
        MatButtonModule,
    ],
    templateUrl: "./ggh.component.html",
})
export class GghComponent {

    // Key generation config
    public dimension: number = 4;
    public basisVectorLength: number = 10;
    public unimodularIterations: number = 10;
    public keyGenSeed: number = 42;

    // Key pair state
    public keyPair: GghKeyPairBean | null = null;

    // Encryption inputs
    public messageInput: string = "";
    public errorRadius: number = 2;
    public encryptSeed: number = 99;

    // Results
    public ciphertextDisplay: string = "";
    public decryptedDisplay: string = "";

    // Internal ciphertext for decrypt
    private ciphertextVector: string[] = [];

    constructor(private gghService: GghBackendRequestService) {
    }

    public generateKeyPair(): void {
        const body: GghCreateKeyPairRequest = {
            dimension: this.dimension,
            basis_vector_length: this.basisVectorLength,
            unimodular_iterations: this.unimodularIterations,
            random_seed: this.keyGenSeed,
        };
        this.gghService.createKeyPair(body).subscribe(kp => {
            this.keyPair = kp;
            this.ciphertextDisplay = "";
            this.decryptedDisplay = "";
            this.ciphertextVector = [];
        });
    }

    public encrypt(): void {
        if (!this.keyPair) {
            return;
        }
        const message = this.parseVector(this.messageInput);
        this.gghService.encrypt({
            message,
            bad_basis: this.keyPair.bad_basis,
            dimension: this.keyPair.dimension,
            error_radius: this.errorRadius,
            random_seed: this.encryptSeed,
        }).subscribe(result => {
            this.ciphertextVector = result.vector;
            this.ciphertextDisplay = result.vector.join(", ");
            this.decryptedDisplay = "";
        });
    }

    public decrypt(): void {
        if (!this.keyPair || this.ciphertextVector.length === 0) {
            return;
        }
        this.gghService.decrypt({
            ciphertext: this.ciphertextVector,
            good_basis: this.keyPair.good_basis,
            good_basis_inverse: this.keyPair.good_basis_inverse,
            unimodular_matrix_inverse: this.keyPair.unimodular_matrix_inverse,
            dimension: this.keyPair.dimension,
        }).subscribe(result => {
            this.decryptedDisplay = result.vector.join(", ");
        });
    }

    public get keyPairDimension(): string {
        return this.keyPair ? String(this.keyPair.dimension) : "-";
    }

    public get hasKeyPair(): boolean {
        return this.keyPair !== null;
    }

    public get hasCiphertext(): boolean {
        return this.ciphertextVector.length > 0;
    }

    private parseVector(input: string): string[] {
        return input
            .split(/[\s,]+/)
            .map(s => s.trim())
            .filter(s => s.length > 0);
    }
}
