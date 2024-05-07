import { ComponentFixture, TestBed, waitForAsync } from '@angular/core/testing';
import { MatFabButton } from '@angular/material/button';
import { MatIcon } from '@angular/material/icon';
import { AddClientButtonComponent } from './add-client-button.component';

describe('AddClientButtonComponent', () => {
  let component: AddClientButtonComponent;
  let fixture: ComponentFixture<AddClientButtonComponent>;

  beforeEach(waitForAsync(() => {
    TestBed.configureTestingModule({
      imports: [
        AddClientButtonComponent,
        MatFabButton,
        MatIcon
      ]
    })
        .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(AddClientButtonComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should emit addClient event when pressed', () => {
    const spy = spyOn(component.addClient, 'emit');
    component.pressed();
    expect(spy).toHaveBeenCalled();
  });
});
