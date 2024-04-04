import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MenezesVanstoneComponent } from './menezes-vanstone.component';

describe('MenezesVanstoneComponent', () => {
  let component: MenezesVanstoneComponent;
  let fixture: ComponentFixture<MenezesVanstoneComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MenezesVanstoneComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MenezesVanstoneComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
