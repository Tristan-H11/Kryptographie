import {ComponentFixture, fakeAsync, TestBed, tick, waitForAsync} from '@angular/core/testing';
import {FormsModule} from '@angular/forms';
import {HttpClientTestingModule} from '@angular/common/http/testing';
import {BrowserAnimationsModule} from '@angular/platform-browser/animations';
import {MatExpansionModule} from '@angular/material/expansion';
import {MatButtonModule} from '@angular/material/button';
import {MatFormFieldModule} from '@angular/material/form-field';
import {MatInputModule} from '@angular/material/input';
import {DisplayCurveComponent, Point} from './display-curve.component';
import {DialogService} from '../services/utility/dialogs.service';

describe('DisplayCurveComponent', () => {
    let component: DisplayCurveComponent;
    let fixture: ComponentFixture<DisplayCurveComponent>;
    let dialogServiceSpy: jasmine.SpyObj<DialogService>;

    beforeEach(waitForAsync(() => {
        const dialogSpy = jasmine.createSpyObj('DialogService', ['showInformationDialog']);

        TestBed.configureTestingModule({
            imports: [
                FormsModule,
                HttpClientTestingModule,
                BrowserAnimationsModule,
                MatExpansionModule,
                MatButtonModule,
                MatFormFieldModule,
                MatInputModule
            ],
            providers: [
                {provide: DialogService, useValue: dialogSpy}
            ]
        }).compileComponents();

        dialogServiceSpy = TestBed.inject(DialogService) as jasmine.SpyObj<DialogService>;
    }));

    beforeEach(() => {
        fixture = TestBed.createComponent(DisplayCurveComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it('should create', () => {
        expect(component).toBeTruthy();
    });

    it('should show information dialog when a_add is NaN', fakeAsync(() => {
        component.a_add = NaN;
        component.add_calculation();
        tick();
        expect(dialogServiceSpy.showInformationDialog).toHaveBeenCalledWith('Please enter a value for A');
    }));

    it('should show information dialog when b_add is NaN', fakeAsync(() => {
        component.b_add = NaN;
        component.add_calculation();
        tick();
        expect(dialogServiceSpy.showInformationDialog).toHaveBeenCalledWith('Please enter a value for B');
    }));

    it('should calculate point addition correctly', fakeAsync(() => {
        component.a_add = -7;
        component.b_add = 10;
        component.P_add = new Point(1, 2);
        component.Q_add = new Point(3, 4);

        component.add_calculation();
        tick();

        expect(component.R_add).toEqual(new Point(-3,  2));
    }));

    it('should calculate point multiplication correctly', fakeAsync(() => {
        component.a_mul = -7;
        component.b_mul = 10;
        component.n_mul = 2;
        component.P_mul = new Point(1, 2);

        component.mul_calculation();
        tick();

        expect(component.Q_mul).toEqual(new Point(-1, -4));
    }));

    it('should show information dialog when a_mul is NaN', fakeAsync(() => {
        component.a_mul = NaN;
        component.mul_calculation();
        tick();
        expect(dialogServiceSpy.showInformationDialog).toHaveBeenCalledWith('Please enter a value for A');
    }));

    it('should show information dialog when b_mul is NaN', fakeAsync(() => {
        component.b_mul = NaN;
        component.mul_calculation();
        tick();
        expect(dialogServiceSpy.showInformationDialog).toHaveBeenCalledWith('Please enter a value for B');
    }));

    it('should show information dialog when n_mul is NaN', fakeAsync(() => {
        component.n_mul = NaN;
        component.mul_calculation();
        tick();
        expect(dialogServiceSpy.showInformationDialog).toHaveBeenCalledWith('Please enter a value for n');
    }));
});
