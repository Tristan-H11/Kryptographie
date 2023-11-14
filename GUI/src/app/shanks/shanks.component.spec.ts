import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ShanksComponent } from './shanks.component';

describe('ShanksComponent', () => {
  let component: ShanksComponent;
  let fixture: ComponentFixture<ShanksComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ShanksComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ShanksComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
