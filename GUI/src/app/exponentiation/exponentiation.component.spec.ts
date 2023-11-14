import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ExponentiationComponent } from './exponentiation.component';

describe('ExponentiationComponent', () => {
  let component: ExponentiationComponent;
  let fixture: ComponentFixture<ExponentiationComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ExponentiationComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ExponentiationComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
