import { ComponentFixture, TestBed } from '@angular/core/testing';

import { CoordinateInputComponent } from './coordinate-input.component';

describe('CoordinateInputComponent', () => {
  let component: CoordinateInputComponent;
  let fixture: ComponentFixture<CoordinateInputComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [CoordinateInputComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(CoordinateInputComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
