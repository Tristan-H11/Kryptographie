import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ClientActionRowComponent } from './client-action-row.component';

describe('ClientActionRowComponent', () => {
  let component: ClientActionRowComponent;
  let fixture: ComponentFixture<ClientActionRowComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ClientActionRowComponent]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(ClientActionRowComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
