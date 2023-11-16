import {NgModule} from '@angular/core';
import {RouterModule, Routes} from "@angular/router";
import {StartseiteComponent} from "./startseite/startseite.component";
import {ExponentiationComponent} from "./exponentiation/exponentiation.component";
import {ExtendedGcdComponent} from "./extendedgcd/extended-gcd.component";
import {ShanksComponent} from "./shanks/shanks.component";
import {ClientComponent} from "./client/client.component";

export const routes: Routes = [
  {path: 'startseite', component: StartseiteComponent},
  {path: 'client/:client', component: ClientComponent},
  {path: 'exponentiation', component: ExponentiationComponent},
  {path: 'extendedGcd', component: ExtendedGcdComponent},
  {path: 'shanks', component: ShanksComponent},
  {path: '', redirectTo: '/startseite', pathMatch: 'full'},
  {path: '**', component: StartseiteComponent}

];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule {
}
