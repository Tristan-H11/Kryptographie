import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AliceComponent } from './alice.component';

describe('AliceComponent', () => {
  let component: AliceComponent;
  let fixture: ComponentFixture<AliceComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AliceComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(AliceComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
