import {ComponentFixture, fakeAsync, TestBed, tick, waitForAsync} from '@angular/core/testing';
import {FormsModule} from '@angular/forms';
import {MatCardModule} from '@angular/material/card';
import {MatExpansionModule} from '@angular/material/expansion';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {HttpClientTestingModule} from "@angular/common/http/testing";
import {BrowserAnimationsModule} from "@angular/platform-browser/animations";
import {MvBackendRequestService} from "../../services/backend-api/mv-backend-request.service";
import {DialogService} from "../../services/utility/dialogs.service";
import {of} from "rxjs";
import {MenezesVanstoneSingleComponent} from "./menezes-vanstone-single.component";
import {MvCipherText, MvDecryptRequest, MvEncryptRequest, MvSignature} from "../../models/mv-beans";

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
        encrypted_message: '',
        points: [{x: '3', y: '8', is_infinite: false}]
    },
    plaintext: 'Test Message 12 ! B',
    signature: {
        r: '',
        s: '',
        string_representation: ''
    },
    sendingTo: undefined,
    receivedFrom: undefined,
    signature_valid: 'ungeprüft'
};
const expectedCipherText: MvCipherText = {
    encrypted_message: 'encrypted_text',
    points: [{x: '3', y: '8', is_infinite: false}]
};
const expectedSignature: MvSignature = {r: '123', s: '456', string_representation: '123,456'};

describe('MenezesVanstoneSingleComponent', () => {
    let component: MenezesVanstoneSingleComponent;
    let fixture: ComponentFixture<MenezesVanstoneSingleComponent>;
    let backendRequestServiceSpy: jasmine.SpyObj<MvBackendRequestService>;
    let dialogServiceSpy: jasmine.SpyObj<DialogService>;

    beforeEach(waitForAsync(() => {
        const backendSpy = jasmine.createSpyObj('MvBackendRequestService', ['encrypt', 'sign', 'decrypt', 'verify', 'createKeyPair']);
        const dialogSpy = jasmine.createSpyObj('DialogService', ['startTimedCalc', 'endTimedCalc']);

        TestBed.configureTestingModule({
            imports: [
                FormsModule,
                MatCardModule,
                MatFormFieldModule,
                MatInputModule,
                MatExpansionModule,
                HttpClientTestingModule,
                BrowserAnimationsModule
            ],
            providers: [
                {provide: MvBackendRequestService, useValue: backendSpy},
                {provide: DialogService, useValue: dialogSpy}
            ]
        }).compileComponents();
        backendRequestServiceSpy = TestBed.inject(MvBackendRequestService) as jasmine.SpyObj<MvBackendRequestService>;
        dialogServiceSpy = TestBed.inject(DialogService) as jasmine.SpyObj<DialogService>;
    }));

    beforeEach(() => {
        fixture = TestBed.createComponent(MenezesVanstoneSingleComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it('should create', () => {
        expect(component).toBeTruthy();
    });

    it('should generate key pair', fakeAsync(() => {
        backendRequestServiceSpy.createKeyPair.and.returnValue(of(source_client.keyPair));
        dialogServiceSpy.startTimedCalc.and.returnValue('loadingKey');
        component.generateKeys();
        tick(); // Simulate asynchronous operation
        expect(backendRequestServiceSpy.createKeyPair).toHaveBeenCalled();
        expect(component.client.keyPair).toBeDefined();
        expect(dialogServiceSpy.endTimedCalc).toHaveBeenCalledWith('loadingKey', 'Schlüsselpaar generiert.');
    }));

    it('should encrypt single message', fakeAsync(() => {
        backendRequestServiceSpy.encrypt.and.returnValue(of(expectedCipherText));
        backendRequestServiceSpy.sign.and.returnValue(of(expectedSignature));
        dialogServiceSpy.startTimedCalc.and.returnValue('loadingKey');
        component.client = source_client;
        component.client.sendingTo = source_client;
        component.client.plaintext = 'Test Message 12 ! B';
        component.encrypt();
        tick();
        expect(backendRequestServiceSpy.encrypt).toHaveBeenCalledOnceWith({
            public_key: source_client.keyPair.public_key,
            message: 'Test Message 12 ! B',
            radix: 55296
        } as MvEncryptRequest);
        expect(component.client.ciphertext).toEqual(expectedCipherText);
        expect(component.client.signature).toEqual(expectedSignature);
        expect(dialogServiceSpy.endTimedCalc).toHaveBeenCalledWith('loadingKey', 'Nachricht verschlüsselt und signiert.');
    }));

    it('should decrypt single message', fakeAsync(() => {
        const encryptedMessage: MvCipherText = {
            encrypted_message: 'encrypted_text',
            points: [{x: '3', y: '8', is_infinite: false}]
        };
        const expectedPlaintext = 'Test Message 12 ! B';
        backendRequestServiceSpy.decrypt.and.returnValue(of({message: expectedPlaintext}));
        backendRequestServiceSpy.verify.and.returnValue(of({message: 'true'}));
        dialogServiceSpy.startTimedCalc.and.returnValue('loadingKey');
        component.client = source_client;
        component.client.ciphertext = encryptedMessage;
        component.decrypt();
        tick();
        expect(backendRequestServiceSpy.decrypt).toHaveBeenCalledOnceWith({
            private_key: source_client.keyPair.private_key,
            cipher_text: encryptedMessage,
            radix: 55296 // Setzen Sie den radix-Wert auf den entsprechenden Wert
        } as MvDecryptRequest);
        expect(component.client.plaintext).toEqual(expectedPlaintext);
        expect(component.client.signature_valid).toEqual('gültig');
        expect(dialogServiceSpy.endTimedCalc).toHaveBeenCalledWith('loadingKey', 'Nachricht entschlüsselt und verifiziert.');
    }));

});
