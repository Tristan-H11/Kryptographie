import {ApplicationConfig} from '@angular/core';
import {provideRouter} from '@angular/router';

import {routes} from './app-routing.module';
import {provideAnimations} from '@angular/platform-browser/animations';
import {provideHttpClient} from "@angular/common/http";

/**
 * Configuration for the application.
 * providers: List of providers for the application.
 * provideAnimations: Provider for the animations.
 * provideHttpClient: Provider for the HttpClient.
 * provideRouter: Provider for the router.
 */
export const appConfig: ApplicationConfig = {
    providers: [provideRouter(routes), provideAnimations(), provideHttpClient()]
};
