import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatButtonModule} from "@angular/material/button";
import {RouterLink, RouterOutlet} from "@angular/router";
import {MatDividerModule} from "@angular/material/divider";
import {MatIconModule} from "@angular/material/icon";

@Component({
  selector: 'app-sidemenu',
  standalone: true,
  imports: [MatSidenavModule, CommonModule, MatButtonModule, RouterOutlet, RouterLink, MatDividerModule, MatIconModule],
  templateUrl: './sidemenu.component.html',
  styleUrl: './sidemenu.component.css'
})
export class SidemenuComponent {
}
