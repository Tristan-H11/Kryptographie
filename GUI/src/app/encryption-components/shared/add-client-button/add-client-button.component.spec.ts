import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddClientButtonComponent } from './add-client-button.component';

describe('AddClientButtonComponent', () => {
  let component: AddClientButtonComponent;
  let fixture: ComponentFixture<AddClientButtonComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AddClientButtonComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(AddClientButtonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
