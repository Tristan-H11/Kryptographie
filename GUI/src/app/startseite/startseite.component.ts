import {Component} from '@angular/core';
import {MatExpansionModule} from '@angular/material/expansion';
import {MatDatepickerModule} from '@angular/material/datepicker';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatIconModule} from '@angular/material/icon';
import {MatButtonModule} from '@angular/material/button';
import {MatNativeDateModule} from '@angular/material/core';
import {FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators} from "@angular/forms";


@Component({
  selector: 'app-startseite',
  standalone: true,
  imports: [
    MatExpansionModule,
    MatIconModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    MatDatepickerModule,
    MatNativeDateModule,
    ReactiveFormsModule,
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

  public onSubmit() {
    // Hier können Sie die Daten verwenden, z.B. an einen Service senden oder in der Konsole ausgeben
    console.log(this.modulbreite, this.zahlensystem, this.random_seed, this.miller_rabin_iterations);
  }
}
