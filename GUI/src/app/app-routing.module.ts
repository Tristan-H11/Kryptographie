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
        path: "menezesVanstoneSingle",
        loadComponent: () => import("./encryption-components/menezes-vanstone-single/menezes-vanstone-single.component").then(m => m.MenezesVanstoneSingleComponent)
    },
    {
        path: "rsa",
        loadComponent: () => import("./encryption-components/rsa/rsa.component").then(m => m.RsaComponent)
    },
    {
        path: "modPow",
        loadComponent: () => import("./math-components/exponentiation/exponentiation.component").then(m => m.ExponentiationComponent)
    },
    {
        path: "extendedGcd",
        loadComponent: () => import("./math-components/extendedgcd/extended-gcd.component").then(m => m.ExtendedGcdComponent)
    },
    {
        path: "shanks",
        loadComponent: () => import("./math-components/shanks/shanks.component").then(m => m.ShanksComponent)
    },
    {
        path: "modularInverse",
        loadComponent: () => import("./math-components/modular-inverse/modular-inverse.component").then(m => m.ModularInverseComponent)
    },
    {
        path: `multiplication`,
        loadComponent: () => import("./math-components/multiplication/multiplication.component").then(m => m.MultiplicationComponent)
    },
    {
        path: "display-curve",
        loadComponent: () => import("./display-curve/display-curve.component").then(m => m.DisplayCurveComponent)
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
