import {Component, ElementRef, OnInit, ViewChild} from '@angular/core';
import functionPlot from 'function-plot';
import {FormsModule} from "@angular/forms";
import {
    MatAccordion,
    MatExpansionPanel,
    MatExpansionPanelActionRow,
    MatExpansionPanelDescription, MatExpansionPanelHeader, MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatButton} from "@angular/material/button";
import {MatFormField, MatLabel} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {DialogService} from "../services/utility/dialogs.service";
import {CoordinateInputComponent} from "./coordinate-input/coordinate-input.component";

abstract class AbstractPoint {
    public x: number | string;
    public y: number | string;

    protected constructor(x: number | string, y: number | string) {
        this.x = x;
        this.y = y;
    }
}

// Konkrete Implementierung der Klasse Point, die von AbstractPoint erbt
class Point extends AbstractPoint {
    constructor(x: number, y: number) {
        super(x, y);  // Aufruf des Konstruktors der abstrakten Klasse
    }
}

// Konkrete Implementierung der Klasse InfinityPoint, die von AbstractPoint erbt
class InfinityPoint extends AbstractPoint {
    constructor() {
        super("Infinity", "Infinity");  // Aufruf des Konstruktors der abstrakten Klasse
    }
}

@Component({
    selector: 'app-display-curve',
    standalone: true,
    imports: [
        FormsModule,
        MatAccordion,
        MatButton,
        MatExpansionPanel,
        MatExpansionPanelActionRow,
        MatExpansionPanelDescription,
        MatExpansionPanelHeader,
        MatExpansionPanelTitle,
        MatFormField,
        MatInput,
        MatLabel,
        CoordinateInputComponent
    ],
    templateUrl: './display-curve.component.html',
    styleUrls: ['./display-curve.component.scss']
})
export class DisplayCurveComponent implements OnInit {

    @ViewChild('plotContainerAdd') plotContainerAdd!: ElementRef;
    @ViewChild('plotContainerMul') plotContainerMul!: ElementRef;

    // Curve for Addition
    public a_add: number = -7;
    public b_add: number = 10;
    // Point on Curve
    public P_add: AbstractPoint = new Point(1, 2);
    public Q_add: AbstractPoint = new Point(3, 4);
    // Result from point addition
    public R_add: AbstractPoint = new Point(0, 0);

    // Curve for Multiplication
    public a_mul: number = -7;
    public b_mul: number = 10;
    public n_mul: number = 2;
    public P_mul: AbstractPoint = new Point(1, 2);
    // Result point from Multiplication
    public Q_mul: AbstractPoint = new Point(-1, -4);

    constructor(private dialogService: DialogService) {
    }

    ngOnInit(): void {
    }

    private checkIfPointIsOnCurve(p: AbstractPoint, a: number, b: number): boolean {
        // y^2 = x^3 + ax + b
        if (p instanceof Point) {
            const x = Number(p.x);
            const y = Number(p.y);
            return Math.pow(y, 2) === Math.pow(x, 3) + a * x + b;
        }
        return true; // Point at infinity is always on the curve
    }

    private checkInfinityPoint(x: AbstractPoint, y: AbstractPoint): boolean {
        // If one of the points is the point at infinity, the result is the other point
        if (x instanceof InfinityPoint && y instanceof Point) {
            return false;
        } else if (x instanceof Point && y instanceof Point) {
            return false;
        }
        return true; // InfinityPoint
    }

    private calculateYCoordinate(p: AbstractPoint): AbstractPoint {
        if (p instanceof Point) {
            const x = Number(p.x);
            p.y = Math.sqrt(Math.pow(x, 3) + this.a_add * x + this.b_add);
            return p;
        }
        return new InfinityPoint();
    }

    private calculatePointAddition(p: AbstractPoint, q: AbstractPoint): AbstractPoint {
        let slope: number;
        let x3: number;
        let y3: number;

        // Check condition 1: Page 57 P+O := O+P := P --> see function checkInfinityPoint
        if (this.checkInfinityPoint(p, q)) {
            return q;
        }
        if (this.checkInfinityPoint(q, p)) {
            return p;
        }

        // Check condition 2: Page 57 P1, P2 ∈ E(IF) mit P1 = (x1, y1), P2 = (x2, y2), x1 = x2 und y1 + y2 = 0 gilt
        // P1 + P2 := O
        const p_x = Number(p.x);
        const p_y = Number(p.y);
        const q_x = Number(q.x);
        const q_y = Number(q.y);

        if (p_x == q_x && p_y + q_y == 0) {
            return new InfinityPoint();
        }
        // Check condition 3: Page 57 SehnenTangentenVerfahren
        if ((p_x != q_x) || (p_y + q_y != 0)) {
            if (p_x != q_x) {
                slope = (q_y - p_y) / (q_x - p_x);
                x3 = Math.pow(slope, 2) - p_x - q_x;
                y3 = -slope * (x3 - p_x) - p_y;
                return new Point(x3, y3);
            }
            if (p_x == q_x && p_y == q_y && q_y != 0) {
                slope = (3 * Math.pow(p_x, 2) + this.a_add) / (2 * p_y);
                x3 = Math.pow(slope, 2) - 2 * p_x;
                y3 = -slope * (x3 - p_x) - p_y;
                return new Point(x3, y3);
            }
        }
        //Default
        return new InfinityPoint();
    }

    public add_calculation() {
        if (isNaN(this.a_add) ) {
            this.dialogService.showInformationDialog("Please enter a value for A");
            return;
        }
        if (isNaN(this.b_add) ) {
            this.dialogService.showInformationDialog("Please enter a value for B");
            return;
        }
        if (this.P_add instanceof InfinityPoint) {
            this.dialogService.showInformationDialog("Please enter a value for P");
            return;
        }
        if (this.Q_add instanceof InfinityPoint) {
            this.dialogService.showInformationDialog("Please enter a value for Q");
            return;
        }
        if (!this.checkIfPointIsOnCurve(this.P_add, this.a_add, this.b_add)) { // checks y^2 = x^3 + ax + b is true
                this.P_add = this.calculateYCoordinate(this.P_add);
            }
        if (!this.checkIfPointIsOnCurve(this.Q_add, this.a_add, this.b_add)) {
            this.Q_add = this.calculateYCoordinate(this.Q_add);
        }

        this.R_add = this.calculatePointAddition(this.P_add, this.Q_add);
        this.pointAdditionPlotCurve();
    }

    public mul_calculation() {
        if (isNaN(this.a_mul) ) {
            this.dialogService.showInformationDialog("Please enter a value for A");
            return;
        }
        if (isNaN(this.b_mul) ) {
            this.dialogService.showInformationDialog("Please enter a value for B");
            return;
        }
        if (isNaN(this.n_mul) ) {
            this.dialogService.showInformationDialog("Please enter a value for n");
            return;
        }
        if (this.P_mul instanceof InfinityPoint) {
            this.dialogService.showInformationDialog("Please enter a value for P");
            return;
        }
        if (!this.checkIfPointIsOnCurve(this.P_mul, this.a_mul, this.b_mul)) {
            this.P_mul = this.calculateYCoordinate(this.P_mul);
        }
        this.Q_mul = this.P_mul;
        for (let i = 1; i < this.n_mul; i++) {
            this.mul_calculation_step();
        }
        this.pointMultiplicationPlotCurve();
    }

    public mul_calculation_step() {
        if (!this.checkIfPointIsOnCurve(this.Q_mul, this.a_mul, this.b_mul)) {
            this.Q_mul = this.calculateYCoordinate(this.Q_mul);
        }
        this.Q_mul = this.calculatePointAddition(this.P_mul, this.Q_mul);
    }

    private pointAdditionPlotCurve() {
        let P_add_x: number;
        let P_add_y: number;
        let Q_add_x: number;
        let Q_add_y: number;
        let R_add_x: number;
        let R_add_y: number;

        if (this.P_add instanceof InfinityPoint) {
            P_add_x = NaN;
            P_add_y = NaN;
        } else {
            P_add_x = Number(this.P_add.x);
            P_add_y = Number(this.P_add.y);
        }

        if (this.Q_add instanceof InfinityPoint) {
            Q_add_x = NaN;
            Q_add_y = NaN;
        } else {
            Q_add_x = Number(this.Q_add.x);
            Q_add_y = Number(this.Q_add.y);
        }

        if (this.R_add instanceof InfinityPoint) {
            R_add_x = NaN;
            R_add_y = NaN;
        } else {
            R_add_x = Number(this.R_add.x);
            R_add_y = Number(this.R_add.y);
        }
        functionPlot({
            target: this.plotContainerAdd.nativeElement,
            width: 800,
            height: 600,
            yAxis: {domain: [-20, 20]},
            xAxis: {domain: [-20, 20]},
            grid: true,
            data: [
                {
                    fn: `y^2 - (x^3 + (${this.a_add}) * x + ${this.b_add})`,
                    fnType: 'implicit',
                    color: 'blue',
                    closed: false,
                },
                {
                    fn: `(${(Q_add_y - P_add_y) / (Q_add_x - P_add_x)}) * x + (${P_add_y - (Q_add_y - P_add_y) /
                    (Q_add_x - P_add_x) * P_add_x})`,
                    fnType: 'linear',
                    graphType: 'polyline',
                    color: 'red'
                },
                {
                    points: [[P_add_x, P_add_y], [Q_add_x, Q_add_y], [R_add_x, R_add_y]],
                    fnType: 'points',
                    graphType: 'scatter',
                    color: 'yellow',
                },
                {
                    vector: [0, 2 * R_add_y],
                    offset: [R_add_x, -R_add_y],
                    fnType: 'vector',
                    graphType: 'polyline',
                    color: 'black'
                },
            ],
        });
    }

    private pointMultiplicationPlotCurve() {
        let P_mul_x: number;
        let P_mul_y: number;
        let Q_mul_x: number;
        let Q_mul_y: number;

        if (this.P_mul instanceof InfinityPoint) {
            P_mul_x = NaN;
            P_mul_y = NaN;
        } else {
            P_mul_x = Number(this.P_mul.x);
            P_mul_y = Number(this.P_mul.y);
        }

        if (this.Q_mul instanceof InfinityPoint) {
            Q_mul_x = NaN;
            Q_mul_y = NaN;
        } else {
            Q_mul_x = Number(this.Q_mul.x);
            Q_mul_y = Number(this.Q_mul.y);
        }
        functionPlot({
            target: this.plotContainerMul.nativeElement,
            width: 800,
            height: 600,
            yAxis: {domain: [-20, 20]},
            xAxis: {domain: [-20, 20]},
            grid: true,
            data: [
                {
                    fn: `y^2 - (x^3 + (${this.a_mul}) * x + ${this.b_mul})`,
                    fnType: 'implicit',
                    color: 'blue',
                    closed: false,
                },
                {
                    fn: `(${(Q_mul_y - P_mul_y) / (Q_mul_x - P_mul_x)}) * x + (${P_mul_y - (Q_mul_y - P_mul_y) /
                    (Q_mul_x - P_mul_x) * P_mul_x})`,
                    fnType: 'linear',
                    graphType: 'polyline',
                    color: 'red'
                },
                {
                    points: [[P_mul_x, P_mul_y], [Q_mul_x, Q_mul_y]],
                    fnType: 'points',
                    graphType: 'scatter',
                    color: 'yellow',
                },
                {
                    vector: [0, 2 * Q_mul_y],
                    offset: [Q_mul_x, -Q_mul_y],
                    fnType: 'vector',
                    graphType: 'polyline',
                    color: 'black'
                },
            ],
        });
    }
}