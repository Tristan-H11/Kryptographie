import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {ExponentiationRoutingModule} from "./exponentiation-routing.module";

@Component({
  selector: 'app-exponentiation',
  standalone: true,
  imports: [CommonModule, ExponentiationRoutingModule],
  templateUrl: './exponentiation.component.html',
  styleUrl: './exponentiation.component.css'
})
export class ExponentiationComponent {

}
