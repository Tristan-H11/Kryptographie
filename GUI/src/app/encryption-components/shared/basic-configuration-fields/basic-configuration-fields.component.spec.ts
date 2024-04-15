import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BasicConfigurationFieldsComponent } from './basic-configuration-fields.component';

describe('BasicConfigurationFieldsComponent', () => {
  let component: BasicConfigurationFieldsComponent;
  let fixture: ComponentFixture<BasicConfigurationFieldsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BasicConfigurationFieldsComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(BasicConfigurationFieldsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
