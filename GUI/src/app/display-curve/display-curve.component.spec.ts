import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DisplayCurveComponent } from './display-curve.component';

describe('DisplayCurveComponent', () => {
  let component: DisplayCurveComponent;
  let fixture: ComponentFixture<DisplayCurveComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DisplayCurveComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(DisplayCurveComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
