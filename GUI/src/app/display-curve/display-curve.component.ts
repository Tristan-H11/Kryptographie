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

class Point {
    constructor(public x: number, public y: number) {
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
        MatLabel
    ],
    templateUrl: './display-curve.component.html',
    styleUrls: ['./display-curve.component.scss']
})
export class DisplayCurveComponent implements OnInit {

    @ViewChild('plotContainerAdd') plotContainerAdd!: ElementRef;
    @ViewChild('plotContainerMul') plotContainerMul!: ElementRef;

    // Curve for Addition
    public a_add = -7;
    public b_add = 10;
    // Point on Curve
    public P_add = new Point(1, 2);
    public Q_add = new Point(3, 4);
    // Result from point addition
    public R_add = new Point(0, 0);

    // Curve for Multiplication
    public a_mul = -7;
    public b_mul = 10;
    public n_mul = 2;
    public P_mul = new Point(1, 2);
    // Result point from Multiplication
    public Q_mul = new Point(-1, -4);

    constructor(private dialogService: DialogService) {
    }

    ngOnInit(): void {
    }

    private checkIfPointIsOnCurve(p: Point, a: number, b: number): boolean {
        // y^2 = x^3 + ax + b
        return Math.pow(p.y, 2) === Math.pow(p.x, 3) + a * p.x + b;
    }

    private checkNeutralElement(p: Point, q: Point): boolean {
        // If one of the points is the point at infinity, the result is the other point
        return (p.x == 0 && p.y == 0) || (q.x == 0 && q.y == 0);
    }

    private calculateYCoordinate(x: number): number {
        return Math.sqrt(Math.pow(x, 3) + this.a_add * x + this.b_add);
    }

    private calculatePointAddition(p: Point, q: Point): Point {
        let slope: number;
        let x3: number;
        let y3: number;
        // Check condition 1: Page 57 P+O := O+P := P --> see function checkNeutralElement
        // Check condition 2: Page 57 P1, P2 ∈ E(IF) mit P1 = (x1, y1), P2 = (x2, y2), x1 = x2 und y1 + y2 = 0 gilt
        // P1 + P2 := O
        if (p.x == q.x && p.y + q.y == 0) {
            return new Point(0, 0);
        }
        // Check condition 3: Page 57 SehnenTangentenVerfahren
        if ((p.x != q.x) || (p.y + q.y != 0)) {
            if (p.x != q.x) {
                slope = (q.y - p.y) / (q.x - p.x);
                x3 = Math.pow(slope, 2) - p.x - q.x;
                y3 = -slope * (x3 - p.x) - p.y;
                return new Point(x3, y3);
            }
            if (p.x == q.x && p.y == q.y && q.y != 0) {
                slope = (3 * Math.pow(p.x, 2) + this.a_add) / (2 * p.y);
                x3 = Math.pow(slope, 2) - 2 * p.x;
                y3 = -slope * (x3 - p.x) - p.y;
                return new Point(x3, y3);
            }
        }
        //Default
        return new Point(0, 0);
    }

    public add_calculation() {
        if (!this.checkIfPointIsOnCurve(this.P_add, this.a_add, this.b_add)) {
            if (!this.checkNeutralElement(this.P_add, this.Q_add)) {
                this.P_add.y = this.calculateYCoordinate(this.P_add.x);
            } else {
                this.dialogService.showErrorDialog("Nullpunkt, kein Diagramm möglich, ändern Sie die Punktwerte.");
                return;
            }
        }
        if (!this.checkIfPointIsOnCurve(this.Q_add, this.a_add, this.b_add)) {
            if (!this.checkNeutralElement(this.P_add, this.Q_add)) {
                this.Q_add.y = this.calculateYCoordinate(this.Q_add.x);
            } else {
                this.dialogService.showErrorDialog("Nullpunkt, kein Diagramm möglich, ändern Sie die Punktwerte.");
                return;
            }
        }
        this.R_add = this.calculatePointAddition(this.P_add, this.Q_add);
        this.pointAdditionPlotCurve();
    }

    public mul_calculation() {
        if (!this.checkIfPointIsOnCurve(this.P_mul, this.a_mul, this.b_mul)) {
            this.P_mul.y = this.calculateYCoordinate(this.P_mul.x);
        }
        this.Q_mul = this.P_mul;
        for (let i = 1; i < this.n_mul; i++) {
            this.mul_calculation_step();
        }
        this.pointMultiplicationPlotCurve();
    }

    public mul_calculation_step() {
        if (!this.checkIfPointIsOnCurve(this.Q_mul, this.a_mul, this.b_mul)) {
            this.Q_mul.y = this.calculateYCoordinate(this.Q_mul.x);
        }
        this.Q_mul = this.calculatePointAddition(this.P_mul, this.Q_mul);
    }

    private pointAdditionPlotCurve() {
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
                    fn: `(${(this.Q_add.y - this.P_add.y) / (this.Q_add.x - this.P_add.x)}) * x + (${this.P_add.y - (this.Q_add.y - this.P_add.y) /
                    (this.Q_add.x - this.P_add.x) * this.P_add.x})`,
                    fnType: 'linear',
                    graphType: 'polyline',
                    color: 'red'
                },
                {
                    points: [[this.P_add.x, this.P_add.y], [this.Q_add.x, this.Q_add.y], [this.R_add.x, this.R_add.y]],
                    fnType: 'points',
                    graphType: 'scatter',
                    color: 'yellow',
                },
                {
                    vector: [0, 2 * this.R_add.y],
                    offset: [this.R_add.x, -this.R_add.y],
                    fnType: 'vector',
                    graphType: 'polyline',
                    color: 'black'
                },
            ],
        });
    }

    private pointMultiplicationPlotCurve() {
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
                    fn: `(${(this.Q_mul.y - this.P_mul.y) / (this.Q_mul.x - this.P_mul.x)}) * x + (${this.P_mul.y - (this.Q_mul.y - this.P_mul.y) /
                    (this.Q_mul.x - this.P_mul.x) * this.P_mul.x})`,
                    fnType: 'linear',
                    graphType: 'polyline',
                    color: 'red'
                },
                {
                    points: [[this.P_mul.x, this.P_mul.y], [this.Q_mul.x, this.Q_mul.y]],
                    fnType: 'points',
                    graphType: 'scatter',
                    color: 'yellow',
                },
                {
                    vector: [0, 2 * this.Q_mul.y],
                    offset: [this.Q_mul.x, -this.Q_mul.y],
                    fnType: 'vector',
                    graphType: 'polyline',
                    color: 'black'
                },
            ],
        });
    }
}
