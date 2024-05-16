import {Component, EventEmitter, Input, Output} from '@angular/core';
import {MatFormField, MatLabel} from "@angular/material/form-field";
import {FormsModule} from "@angular/forms";
import {MatInput} from "@angular/material/input";
import {NgIf} from "@angular/common";

@Component({
  selector: 'app-coordinate-input',
  standalone: true,
  imports: [
    MatLabel,
    MatFormField,
    FormsModule,
    MatInput,
    NgIf
  ],
  templateUrl: './coordinate-input.component.html',
  styleUrl: './coordinate-input.component.scss'
})
export class CoordinateInputComponent {
  @Input() labelX: string = "";
  @Input() labelY: string = "";
  @Input() placeholderX: string = "";
  @Input() placeholderY: string = "";
  @Input() readOnly: boolean = false;

  _val_a: number | undefined = undefined;
  _val_b: number | undefined = undefined;
  _point_x: number | string = "";
  _point_y: number | string = "";

  @Input() set val_a(value: number) {
    this._val_a = value;
  }

  @Input() set val_b(value: number) {
    this._val_b = value;
  }

  @Input() set point_x(value: number | string) {
    if (value === "Infinity") {
      this._point_x = "Infinity";
    } else {
      this._point_x = parseFloat(value as string);
    }
  }

  @Input() set point_y(value: number | string) {
    if (value === "Infinity") {
      this._point_y = "Infinity";
    } else {
      this._point_y = parseFloat(value as string);
    }
  }

  @Output() val_aChange = new EventEmitter<number>();
  @Output() val_bChange = new EventEmitter<number>();
  @Output() point_xChange = new EventEmitter<number | string>();
  @Output() point_yChange = new EventEmitter<number | string>();

  constructor() { }

  onValAChange(value: string) {
    this.val_aChange.emit(parseFloat(value));
  }

  onValBChange(value: string) {
    this.val_bChange.emit(parseFloat(value));
  }

  onPointXChange(value: string) {
    if (value === "Infinity") {
      this.point_xChange.emit("Infinity");
    } else {
      this.point_xChange.emit(parseFloat(value));
    }
  }

  onPointYChange(value: string) {
    if (value === "Infinity") {
      this.point_yChange.emit("Infinity");
    } else {
      this.point_yChange.emit(parseFloat(value));
    }
  }
}