import { ComponentFixture, TestBed } from '@angular/core/testing';

import { StartseiteComponent } from './startseite.component';

describe('StartseiteComponent', () => {
  let component: StartseiteComponent;
  let fixture: ComponentFixture<StartseiteComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [StartseiteComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(StartseiteComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
