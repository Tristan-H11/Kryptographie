import {ComponentFixture, fakeAsync, TestBed, tick, waitForAsync} from '@angular/core/testing';
import { FormsModule } from '@angular/forms';
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MvClientPanelComponent } from './mv-client-panel.component';
import {HttpClient} from "@angular/common/http";
import {HttpClientTestingModule} from "@angular/common/http/testing";
import {BrowserAnimationsModule} from "@angular/platform-browser/animations";
import {MvCipherText, MvEncryptRequest, MvSignature} from "../../../models/mv-beans";
import {MvBackendRequestService} from "../../../services/backend-api/mv-backend-request.service";
import {DialogService} from "../../../services/utility/dialogs.service";
import {of} from "rxjs";

describe('MvClientPanelComponent', () => {
  let component: MvClientPanelComponent;
  let fixture: ComponentFixture<MvClientPanelComponent>;
  let backendRequestServiceSpy: jasmine.SpyObj<MvBackendRequestService>;
  let dialogServiceSpy: jasmine.SpyObj<DialogService>;

  beforeEach(waitForAsync(() => {
    const backendSpy = jasmine.createSpyObj('MvBackendRequestService', ['encrypt', 'sign']);
    const dialogSpy = jasmine.createSpyObj('DialogService', ['startTimedCalc', 'endTimedCalc']);

    TestBed.configureTestingModule({
      imports: [
        FormsModule,
        MatCardModule,
        MatFormFieldModule,
        MatInputModule,
        HttpClientTestingModule,
        BrowserAnimationsModule
      ],
      providers: [
        { provide: MvBackendRequestService, useValue: backendSpy },
        { provide: DialogService, useValue: dialogSpy }
      ]
    })
        .compileComponents();
    backendRequestServiceSpy = TestBed.inject(MvBackendRequestService) as jasmine.SpyObj<MvBackendRequestService>;
    dialogServiceSpy = TestBed.inject(DialogService) as jasmine.SpyObj<DialogService>;
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(MvClientPanelComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should encrypt message when send is called', fakeAsync(() => {
    const source_client = {
      name: 'source_client',
      keyPair: {
        public_key: {
          curve: {
            a: 0,
            prime: '13',
            order_of_subgroup: '7',
            generator: {
              x: '3',
              y: '8',
              is_infinite: false
            }
          },
          y: {
            x: '3',
            y: '8',
            is_infinite: false
          }
        },
        private_key: {
          curve: {
            a: 0,
            prime: '19',
            order_of_subgroup: '13',
            generator: {
              x: '2',
              y: '7',
              is_infinite: false
            }
          },
          x: '3'
        }
      },
      ciphertext: {
        encrypted_message: 'encrypted_text',
        points: [{ x: '3', y: '8', is_infinite: false }]
      },
      plaintext: 'Test Message 12 ! B',
      signature: {
        r: '123',
        s: '456'
      },
      sendingTo: undefined,
      receivedFrom: undefined,
      signature_valid: 'ungeprüft'
    };

    const expectedCipherText: MvCipherText = {
      encrypted_message: 'encrypted_text',
      points: [{ x: '3', y: '8', is_infinite: false }]
    };
    const expectedSignature: MvSignature = { r: '123', s: '456' };

    backendRequestServiceSpy.encrypt.and.returnValue(of(expectedCipherText));
    backendRequestServiceSpy.sign.and.returnValue(of(expectedSignature));
    dialogServiceSpy.startTimedCalc.and.returnValue('loadingKey');

    component.client = source_client;
    component.client.sendingTo = source_client;
    component.encrypt();
    tick(); // Simuliert asynchrone Operationen

    expect(backendRequestServiceSpy.encrypt).toHaveBeenCalledOnceWith({
      public_key: source_client.keyPair.public_key,
      message: source_client.plaintext,
      radix: 0 // Setzen Sie den Radix auf den erwarteten Wert
    } as MvEncryptRequest);
    expect(component.client.ciphertext).toEqual(expectedCipherText);
    expect(component.client.signature).toEqual(expectedSignature);
    expect(dialogServiceSpy.endTimedCalc).toHaveBeenCalledWith('loadingKey', 'Nachricht verschlüsselt und signiert.');
  }));
});
