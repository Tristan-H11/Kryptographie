import {Component} from '@angular/core';
import {MatExpansionModule} from '@angular/material/expansion';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatButtonModule} from '@angular/material/button';
import {FormsModule} from "@angular/forms";
import {StartseiteRoutingModule} from "./startseite-routing.module";


@Component({
  selector: 'app-startseite',
  standalone: true,
  imports: [
    StartseiteRoutingModule,
    MatExpansionModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    FormsModule,
  ],
  templateUrl: './startseite.component.html',
  styleUrl: './startseite.component.css'
})
export class StartseiteComponent {

  public modulbreite: number = 4096;
  public zahlensystem: number = 55296;
  public random_seed: number = 13;
  public miller_rabin_iterations: number = 100;
  public modul_alice: string = "";
  public e_alice: string = "";
  public modul_bob: string = "";
  public e_bob: string = "";

  public onSubmit() {
    // Hier können Sie die Daten verwenden, z.B. an einen Service senden oder in der Konsole ausgeben
    console.log(this.modulbreite, this.zahlensystem, this.random_seed, this.miller_rabin_iterations);
    this.modul_alice = this.modulbreite.toString();
  }
}
