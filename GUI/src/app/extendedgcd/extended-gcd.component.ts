import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {ExtendedGcdRoutingModule} from "./extended-gcd-routing.module";
import {MatButtonModule} from "@angular/material/button";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {BackendRequestService} from "../services/backend-api/backend-request.service";
import {createExtendedEuclidRequestFrom} from "../models/extended-euclid-request";

@Component({
  selector: 'app-extended-gcd',
  standalone: true,
  imports: [CommonModule, ExtendedGcdRoutingModule, MatButtonModule, MatExpansionModule, MatFormFieldModule, MatInputModule, ReactiveFormsModule, FormsModule],
  templateUrl: './extended-gcd.component.html',
  styleUrl: './extended-gcd.component.scss'
})
export class ExtendedGcdComponent {
  public ggT: string= "";
  public parameterA: string = "";
  public parameterB: string = "";
  public coefficientX: string = "";
  public coefficientY: string = "";

  constructor(private backendRequestService: BackendRequestService) {
  }

  /**
   * Berechnet den ggT.
   */
  public calculate() {

    const body = createExtendedEuclidRequestFrom(this.parameterA, this.parameterB);

    this.backendRequestService.extendedGcd(body).then(result => {
      this.ggT = result.ggt;
      this.coefficientX = result.x;
      this.coefficientY = result.y;
    });
  }
}
