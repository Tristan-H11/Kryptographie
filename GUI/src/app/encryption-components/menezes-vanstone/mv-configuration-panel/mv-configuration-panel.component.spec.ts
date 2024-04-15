import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MvConfigurationPanelComponent } from './mv-configuration-panel.component';

describe('MvConfigurationPanelComponent', () => {
  let component: MvConfigurationPanelComponent;
  let fixture: ComponentFixture<MvConfigurationPanelComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MvConfigurationPanelComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MvConfigurationPanelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
