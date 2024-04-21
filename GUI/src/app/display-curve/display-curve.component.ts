import { AfterViewInit, Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import functionPlot from 'function-plot';
import { FormsModule } from "@angular/forms";
import {
    MatAccordion,
    MatExpansionPanel,
    MatExpansionPanelActionRow,
    MatExpansionPanelDescription, MatExpansionPanelHeader, MatExpansionPanelTitle
} from "@angular/material/expansion";
import {MatButton} from "@angular/material/button";
import {MatFormField, MatLabel} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";

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
    public a = -7;
    public b = 10;
    // Point on Curve
    public P = { x: 1, y: 2 };
    public Q = { x: 3, y: 4 };
    // Result from point addition
    public R = { x: 0, y: 0 };

    // Curve for Multiplication
    public a2 = -7;
    public b2 = 10;
    public n2 = 2;
    public P2 = { x: 1, y: 2 };
    // Result point from Multiplication
    public Q2 = { x: -1, y: -4 };

    constructor() { }

    ngOnInit(): void { }

    private checkIfPointIsOnCurve(x: number, y: number): boolean {
        // y^2 = x^3 + ax + b
        if (x == 0 && y == 0) {
            return true; // point at infinity
        }
        return Math.pow(y, 2) === Math.pow(x, 3) + this.a * x + this.b;
    }

    private calculateYCoordinate(x: number): number {
        return Math.sqrt(Math.pow(x, 3) + this.a * x + this.b);
    }

    private calculateAdditionPointR(p: { x: number, y: number }, q: { x: number, y: number }): { x: number, y: number } {
        let slope: number;
        let x3: number;
        let y3: number;

        // Check if P or Q is the point at infinity
        if ((p.x == 0 && p.y == 0) || (q.x == 0 && q.y == 0)) {
            return (p.x == 0 && p.y == 0) ? q : p;
        }

        // Check if the tangent at P is vertical
        if (p.x == q.x && p.y == q.y) {
            if (p.y == 0) {
                return { x: 0, y: 0 };
            }
        }

        // Check if p.x != q.x and p.y + q.y != 0
        if (p.x != q.x && p.y + q.y != 0) {
            if (p.x != q.x) {
                slope = (q.y - p.y) / (q.x - p.x);
                x3 = Math.pow(slope, 2) - p.x - q.x;
                y3 = -slope * (x3 - p.x) - p.y;
                return { x: x3, y: y3 };
            }
            if (p.x == q.x && p.y == q.y && q.y != 0) {
                slope = (3 * Math.pow(p.x, 2) + this.a) / (2 * p.y);
                x3 = Math.pow(slope, 2) - 2 * p.x;
                y3 = -slope * (x3 - p.x) - p.y;
                return { x: x3, y: y3 };
            }
        }
        return { x: 0, y: 0 };
    }

    private calculateMultiplicationPointR(p: { x: number, y: number }, n: number): { x: number, y: number } {
        // let q = p;
        //
        // for (let i = 1; i <= n; i++) {
        //     q = this.calculateAdditionPointR(p, q);
        // }
        // return q;

        return { x: 0, y: 0 };
    }

    public add_calculation() {
        if (!this.checkIfPointIsOnCurve(this.P.x, this.P.y)) {
            this.P.y = this.calculateYCoordinate(this.P.x);
        }
        if (!this.checkIfPointIsOnCurve(this.Q.x, this.Q.y)) {
            this.Q.y = this.calculateYCoordinate(this.Q.x);
        }
        this.R = this.calculateAdditionPointR(this.P, this.Q);
        this.pointAdditionPlotCurve();
    }

    public mul_calculation() {
        // this.Q2 = this.calculateMultiplicationPointR(this.P2, this.n2);
        // this.pointMultiplicationPlotCurve();
    }

    private pointAdditionPlotCurve() {
        functionPlot({
            target: this.plotContainerAdd.nativeElement,
            width: 800,
            height: 600,
            yAxis: { domain: [-20, 20] },
            xAxis: { domain: [-20, 20] },
            grid: true,
            data: [
                {
                    fn: `y^2 - (x^3 + (${this.a}) * x + ${this.b})`,
                    fnType: 'implicit',
                    color: 'blue',
                    closed: false,
                },
                {
                    fn: `(${(this.Q.y - this.P.y) / (this.Q.x - this.P.x)}) * x + (${this.P.y - (this.Q.y - this.P.y) /
                    (this.Q.x - this.P.x) * this.P.x})`,
                    fnType: 'linear',
                    graphType: 'polyline',
                    color: 'red'
                },
                {
                    points: [[this.P.x, this.P.y], [this.Q.x, this.Q.y], [this.R.x, this.R.y]],
                    fnType: 'points',
                    graphType: 'scatter',
                    color: 'yellow',
                },
                {
                    vector: [0, 2 * this.R.y],
                    offset: [this.R.x, -this.R.y],
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
            yAxis: { domain: [-20, 20] },
            xAxis: { domain: [-20, 20] },
            grid: true,
            data: [
                {
                    fn: `y^2 - (x^3 + (${this.a2}) * x + ${this.b2})`,
                    fnType: 'implicit',
                    color: 'blue',
                    closed: false,
                },
                {
                    points: [[this.P2.x, this.P2.y], [this.Q2.x, this.Q2.y]],
                    fnType: 'points',
                    graphType: 'scatter',
                    color: 'yellow',
                },
            ],
        });
    }
}
