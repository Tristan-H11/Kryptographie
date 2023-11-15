import {Component, OnInit} from '@angular/core';
import {MatExpansionModule} from '@angular/material/expansion';
import {MatInputModule} from '@angular/material/input';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatButtonModule} from '@angular/material/button';
import {FormsModule} from "@angular/forms";
import {StartseiteRoutingModule} from "./startseite-routing.module";
import {ClientEnum} from "../models/client-enum";
import {KeyManagementService} from "../services/key-management.service";
import {createKeyPairRequestFrom} from "../models/create-key-pair-request";


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
export class StartseiteComponent implements OnInit {

  public modulbreite: number = 4096;
  public zahlensystem: number = 55296;
  public random_seed: number = 13;
  public miller_rabin_iterations: number = 100;
  public modul_alice: string = "";
  public e_alice: string = "";
  public modul_bob: string = "";
  public e_bob: string = "";

  constructor(private keyService: KeyManagementService) {
  }

  public generateKeys(client: ClientEnum) {
    let requestContent = createKeyPairRequestFrom(
      this.modulbreite,
      this.miller_rabin_iterations,
      this.random_seed,
      this.zahlensystem
    )
    this.keyService.generateKeyPair(requestContent, client);
  }

  protected readonly ClientEnum = ClientEnum;

  ngOnInit(): void {
    this.keyService.getKeyPair(ClientEnum.Alice).subscribe(keyPair => {
      this.modul_alice = keyPair.public_key.modulus;
      this.e_alice = keyPair.public_key.e;
    });

    this.keyService.getKeyPair(ClientEnum.Bob).subscribe(keyPair => {
      this.modul_bob = keyPair.public_key.modulus;
      this.e_bob = keyPair.public_key.e;
    });
  }
}
