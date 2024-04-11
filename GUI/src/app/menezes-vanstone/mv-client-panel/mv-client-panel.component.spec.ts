import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MvClientPanelComponent } from './mv-client-panel.component';

describe('MvClientPanelComponent', () => {
  let component: MvClientPanelComponent;
  let fixture: ComponentFixture<MvClientPanelComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MvClientPanelComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(MvClientPanelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
