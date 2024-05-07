import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';
import { AddClientButtonComponent } from '../shared/add-client-button/add-client-button.component';
import { MatButtonModule } from '@angular/material/button';
import { MatAccordion } from '@angular/material/expansion';
import { MvBasicsPanelComponent } from '../menezes-vanstone/mv-basics-panel/mv-basics-panel.component';
import { MvClientPanelComponent } from '../menezes-vanstone/mv-client-panel/mv-client-panel.component';
import { MvConfigurationPanelComponent } from '../menezes-vanstone/mv-configuration-panel/mv-configuration-panel.component';
import { NgForOf } from '@angular/common';
import { RsaBasicsPanelComponent } from './rsa-basics-panel/rsa-basics-panel.component';
import { RsaConfigurationPanelComponent } from './rsa-configuration-panel/rsa-configuration-panel.component';
import { RsaClientPanelComponent } from './rsa-client-panel/rsa-client-panel.component';
import { RsaComponent } from './rsa.component';
import {HttpClientTestingModule} from "@angular/common/http/testing";
import {BrowserAnimationsModule} from "@angular/platform-browser/animations";

describe('RsaComponent', () => {
  let component: RsaComponent;
  let fixture: ComponentFixture<RsaComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      imports: [
        MatButtonModule,
        HttpClientTestingModule,
        BrowserAnimationsModule
      ]
    })
        .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RsaComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

});
