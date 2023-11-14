import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {ExtendedGcdRoutingModule} from "./extended-gcd-routing.module";

@Component({
  selector: 'app-extended-gcd',
  standalone: true,
  imports: [CommonModule, ExtendedGcdRoutingModule],
  templateUrl: './extended-gcd.component.html',
  styleUrl: './extended-gcd.component.css'
})
export class ExtendedGcdComponent {

}
