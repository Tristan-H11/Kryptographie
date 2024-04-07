import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {HomepageComponent} from "./homepage/homepage.component";
import {ExponentiationComponent} from "./exponentiation/exponentiation.component";
import {ExtendedGcdComponent} from "./extendedgcd/extended-gcd.component";
import {ShanksComponent} from "./shanks/shanks.component";
import {ClientComponent} from "./client/client.component";
import {ModularInverseComponent} from "./modular-inverse/modular-inverse.component";
import {MultiplicationComponent} from "./multiplication/multiplication.component";
import {MenezesVanstoneComponent} from "./menezes-vanstone/menezes-vanstone.component";
import {RsaComponent} from "./rsa/rsa.component";

/**
 * Defines the routes of the application.
 * The routing takes place in navbar.component.html and homepage.html
 */
export const routes: Routes = [
    {path: "homepage", component: HomepageComponent},
    {path: "client/:client", component: ClientComponent},
    {path: "menezesVanstone", component: MenezesVanstoneComponent},
    {path: "rsa", component: RsaComponent},
    {path: "exponentiation", component: ExponentiationComponent},
    {path: "extendedGcd", component: ExtendedGcdComponent},
    {path: "shanks", component: ShanksComponent},
    {path: "modularInverse", component: ModularInverseComponent},
    {path: `multiplication`, component: MultiplicationComponent},
    {path: "", redirectTo: "/homepage", pathMatch: "full"},
    {path: "**", component: HomepageComponent}
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule]
})

export class AppRoutingModule {
}
