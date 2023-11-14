import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ExtendedGcdComponent } from './extended-gcd.component';

describe('ExtendedgcdComponent', () => {
  let component: ExtendedGcdComponent;
  let fixture: ComponentFixture<ExtendedGcdComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ExtendedGcdComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ExtendedGcdComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
