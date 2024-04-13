import { ComponentFixture, TestBed } from '@angular/core/testing';

import { RsaBasicsPanelComponent } from './rsa-basics-panel.component';

describe('RsaBasicsPanelComponent', () => {
  let component: RsaBasicsPanelComponent;
  let fixture: ComponentFixture<RsaBasicsPanelComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [RsaBasicsPanelComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(RsaBasicsPanelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
