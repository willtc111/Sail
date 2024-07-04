
export class RollingAverage {
  historySize: number;
  values: number[];
  average: number | undefined;
  index: number;

  constructor() {
    this.historySize = 50;
    this.values = [];
    this.average = 0;
    this.index = 0;
  }

  add(value: number) {
    if (this.values.length < this.historySize) {
      if (this.values.length == 0) {
        // empty history
        this.average = value;
      } else {
        // partially full history
        this.average = (this.average! * this.values.length + value) / (this.values.length + 1);
      }
      this.values.push(value);
      this.index = this.values.length;
    } else {
      // full history
      let old = this.values[this.index];
      this.values[this.index] = value;
      this.average = this.average! - old / this.historySize + value / this.historySize;
      this.index++;
    }
    if (this.index >= this.historySize) {
      this.index = 0;
    }
  }

  get(): number | undefined {
    return this.average;
  }

  clear() {
    this.values = [];
    this.index = 0;
  }
}