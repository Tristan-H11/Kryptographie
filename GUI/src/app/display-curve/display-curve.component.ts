import { AfterViewInit, Component, ElementRef, OnInit, ViewChild } from '@angular/core';
import functionPlot from 'function-plot';
import {
    MatAccordion,
    MatExpansionPanel,
    MatExpansionPanelActionRow,
    MatExpansionPanelDescription, MatExpansionPanelHeader, MatExpansionPanelTitle,
} from "@angular/material/expansion";
import {MatButton} from "@angular/material/button";
import {MatFormField, MatLabel} from "@angular/material/form-field";
import {MatInput} from "@angular/material/input";
import {FormsModule} from "@angular/forms";

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
    styleUrl: './display-curve.component.scss'
})
export class DisplayCurveComponent implements OnInit, AfterViewInit {

    @ViewChild('plotContainer') plotContainer!: ElementRef;

    public a = -7;
    public b = 10;
    public P = { x: 1, y: 2 };
    public Q = { x: 3, y: 4 };
    public R = { x: 0, y:  0};

    constructor() {
    }

    ngOnInit(): void {
    }

    // needed for mouse movements on the graph
    ngAfterViewInit(): void {
    }

    private pointAdditionPlotCurve() {
        functionPlot({
            target: this.plotContainer.nativeElement,
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
            annotations: [
                {
                    x: this.P.x,
                    y: this.P.y,
                    text: `P (${this.P.x.toFixed(0)}, ${this.P.y.toFixed(0)})`
                },
                {
                    x: this.Q.x,
                    y: this.Q.y,
                    text: `Q (${this.Q.x.toFixed(0)}, ${this.Q.y.toFixed(0)})`
                },
                {
                    x: this.R.x,
                    y: this.R.y,
                    text: `R (${this.R.x.toFixed(0)}, ${this.R.y.toFixed(0)})`
                }
            ]
        });
    }
    private calculatePointR(p: {x: number, y: number} , q: {x: number, y: number}): void {
        let slope: number;
        let x3: number;
        let y3: number;

        // Check if P or Q is the point at infinity
        if ((p.x == 0 && p.y == 0) || (q.x == 0 && q.y == 0)) {
            this.R = (p.x == 0 && p.y == 0) ? q : p;
            return;
        }

        // Check if the tangent at P is vertical
        if (p.x == q.x && p.y == q.y) {
            if (p.y == 0) {
                this.R = {x: 0, y: 0};  // infinity
                return;
            }
        }

        // Check if p.x != q.x and p.y + q.y != 0
        if (p.x != q.x && p.y + q.y != 0) {
            if (p.x != q.x) {
                slope = (q.y - p.y) / (q.x - p.x);
                x3 = Math.pow(slope, 2) - p.x - q.x;
                y3 = -slope * (x3 - p.x) - p.y;
                this.R = {x: x3, y: y3};
                return;
            }
            if (p.x == q.x && p.y == q.y && q.y != 0){
                slope = (3 * Math.pow(p.x, 2) + this.a) / (2 * p.y);
                x3 = Math.pow(slope, 2) - 2 * p.x;
                y3 = -slope * (x3 - p.x) - p.y;
                this.R = {x: x3, y: y3};
                return;
            }
        }
    }

    public calculate() {
        this.calculatePointR(this.P, this.Q);
        this.pointAdditionPlotCurve();
    }
}
