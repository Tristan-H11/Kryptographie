import { NgModule } from '@angular/core';
import {RouterModule, Routes} from "@angular/router";
import {StartseiteComponent} from "./startseite/startseite.component";
import {AliceComponent} from "./alice/alice.component";
import {BobComponent} from "./bob/bob.component";

export const routes: Routes = [
  { path: 'startseite', component: StartseiteComponent },
  { path: 'alice', component: AliceComponent},
  { path: 'bob', component: BobComponent},
  { path: '', redirectTo: '/startseite', pathMatch: 'full' },
  { path: '**', component: StartseiteComponent }

];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
