import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MvBasicsPanelComponent } from './mv-basics-panel.component';

describe('MvBasicsPanelComponent', () => {
  let component: MvBasicsPanelComponent;
  let fixture: ComponentFixture<MvBasicsPanelComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MvBasicsPanelComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MvBasicsPanelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
