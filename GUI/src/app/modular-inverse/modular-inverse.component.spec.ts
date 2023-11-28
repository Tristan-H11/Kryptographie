import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ModularInverseComponent } from './modular-inverse.component';

describe('ModularInverseComponent', () => {
  let component: ModularInverseComponent;
  let fixture: ComponentFixture<ModularInverseComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ModularInverseComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ModularInverseComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
