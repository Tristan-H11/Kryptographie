import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {ExponentiationRoutingModule} from "./exponentiation-routing.module";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {BackendRequestService} from "../services/backend-api/backend-request.service";
import {createExponentiationRequestFrom} from "../models/exponentiation-request";
import {MatButtonModule} from "@angular/material/button";

@Component({
  selector: 'app-exponentiation',
  standalone: true,
  imports: [CommonModule, ExponentiationRoutingModule, MatExpansionModule, MatFormFieldModule, MatInputModule, ReactiveFormsModule, FormsModule, MatButtonModule],
  templateUrl: './exponentiation.component.html',
  styleUrl: './exponentiation.component.css'
})
export class ExponentiationComponent {

  constructor(private backendRequestService: BackendRequestService) {
  }

  public exponent = "";
  public base = "";
  public modulus = "";

  public result = "";

  /**
   * Berechnet die Exponentiation.
   */
  public calculate() {
    let body = createExponentiationRequestFrom(this.exponent, this.base, this.modulus);
    this.backendRequestService.exponentiation(body).then(result => {
      this.result = result.message;
    });
  }
}
