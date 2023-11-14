import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { AppRoutingModule } from './app-routing.module';
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatInputModule} from "@angular/material/input";



@NgModule({
  declarations: [],
  imports: [
    CommonModule,
    AppRoutingModule,
    MatFormFieldModule,
    MatInputModule
  ]
})
export class AppModule { }
