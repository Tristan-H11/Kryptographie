import {ComponentFixture, fakeAsync, TestBed, tick, waitForAsync} from '@angular/core/testing';
import {FormsModule} from '@angular/forms';
import {MatCardModule} from '@angular/material/card';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {HttpClientTestingModule} from "@angular/common/http/testing";
import {BrowserAnimationsModule} from "@angular/platform-browser/animations";
import {MvCipherText, MvDecryptRequest, MvEncryptRequest, MvSignature, MvVerifyRequest} from "../../../models/mv-beans";
import {MvBackendRequestService} from "../../../services/backend-api/mv-backend-request.service";
import {DialogService} from "../../../services/utility/dialogs.service";
import {of} from "rxjs";
import {MvClientPanelComponent} from "./mv-client-panel.component";

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
        points: [{x: '3', y: '8', is_infinite: false}]
    },
    plaintext: 'Test Message 12 ! B',
    signature: {
        r: '123',
        s: '456',
        string_representation: '123,456'
    },
    sendingTo: undefined,
    receivedFrom: undefined,
    signature_valid: 'ungeprüft'
};
const target_client = {
    name: 'target_client',
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
        points: [{x: '3', y: '8', is_infinite: false}]
    },
    plaintext: 'Test Message 12 ! B',
    signature: {
        r: '123',
        s: '456',
        string_representation: '123,456'
    },
    sendingTo: undefined,
    receivedFrom: source_client,
    signature_valid: 'ungeprüft'
};
const expectedCipherText: MvCipherText = {
    encrypted_message: 'encrypted_text',
    points: [{x: '3', y: '8', is_infinite: false}]
};
const expectedSignature: MvSignature = {r: '123', s: '456', string_representation: '123,456'};
const expectedPlaintext = 'Test Message 12 ! B';
const expectedVerifyResponse = {message: 'true'};

describe('MvClientPanelComponent', () => {
    let component: MvClientPanelComponent;
    let fixture: ComponentFixture<MvClientPanelComponent>;
    let backendRequestServiceSpy: jasmine.SpyObj<MvBackendRequestService>;
    let dialogServiceSpy: jasmine.SpyObj<DialogService>;

    beforeEach(waitForAsync(() => {
        const backendSpy = jasmine.createSpyObj('MvBackendRequestService', ['encrypt', 'sign', 'decrypt', 'verify']);
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
                {provide: MvBackendRequestService, useValue: backendSpy},
                {provide: DialogService, useValue: dialogSpy}
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

    it('should decrypt message', fakeAsync(() => {
        backendRequestServiceSpy.decrypt.and.returnValue(of({message: expectedPlaintext}));
        backendRequestServiceSpy.verify.and.returnValue(of(expectedVerifyResponse));
        dialogServiceSpy.startTimedCalc.and.returnValue('loadingCalcKey');

        component.client = target_client;
        component.decrypt();
        tick();

        // Assert
        expect(backendRequestServiceSpy.decrypt).toHaveBeenCalledOnceWith({
            private_key: target_client.keyPair.private_key,
            cipher_text: target_client.ciphertext,
            radix: 0
        } as MvDecryptRequest);
        expect(component.client.plaintext).toEqual(expectedPlaintext);
        expect(backendRequestServiceSpy.verify).toHaveBeenCalledOnceWith({
            public_key: source_client.keyPair.public_key,
            message: expectedPlaintext,
            signature: target_client.signature
        } as MvVerifyRequest);
        expect(component.client.signature_valid).toEqual('gültig');
        expect(dialogServiceSpy.endTimedCalc).toHaveBeenCalledWith('loadingCalcKey', 'Nachricht entschlüsselt und verifiziert.');
    }));

    it('should not encrypt message when sendingTo is not set', fakeAsync(() => {
        component.client = source_client;
        component.client.sendingTo = undefined;  // No target client
        component.encrypt();
        tick();
        expect(backendRequestServiceSpy.encrypt).not.toHaveBeenCalled();
        expect(dialogServiceSpy.endTimedCalc).not.toHaveBeenCalled();
    }));


});
