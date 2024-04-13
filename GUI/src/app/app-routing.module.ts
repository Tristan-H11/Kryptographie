import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";

export const routes: Routes = [
    {
        path: "homepage",
        loadComponent: () => import("./homepage/homepage.component").then(m => m.HomepageComponent)
    },
    {
        path: "menezesVanstone",
        loadComponent: () => import("./encryption-components/menezes-vanstone/menezes-vanstone.component").then(m => m.MenezesVanstoneComponent)
    },
    {
        path: "rsa",
        loadComponent: () => import("./encryption-components/rsa/rsa.component").then(m => m.RsaComponent)
    },
    {
        path: "exponentiation",
        loadComponent: () => import("./exponentiation/exponentiation.component").then(m => m.ExponentiationComponent)
    },
    {
        path: "extendedGcd",
        loadComponent: () => import("./extendedgcd/extended-gcd.component").then(m => m.ExtendedGcdComponent)
    },
    {
        path: "shanks",
        loadComponent: () => import("./shanks/shanks.component").then(m => m.ShanksComponent)
    },
    {
        path: "modularInverse",
        loadComponent: () => import("./modular-inverse/modular-inverse.component").then(m => m.ModularInverseComponent)
    },
    {
        path: `multiplication`,
        loadComponent: () => import("./multiplication/multiplication.component").then(m => m.MultiplicationComponent)
    },
    {
        path: "",
        redirectTo: "/homepage",
        pathMatch: "full"
    },
    {
        path: "**",
        redirectTo: "/homepage"
    }
];

@NgModule({
    imports: [RouterModule.forRoot(routes)],
    exports: [RouterModule]
})

export class AppRoutingModule {
}
