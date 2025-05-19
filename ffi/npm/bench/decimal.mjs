import Benchmark from 'benchmark';

import { Decimal, DecimalSign } from "icu4x"

let suite = new Benchmark.Suite();

suite = suite.add("Decimal.fromBigInt", () => {
  (Decimal.fromBigInt(BigInt(1234))).underlying > 0;
});

const decimal = Decimal.fromBigInt(BigInt(1234));
decimal.multiplyPow10(-2);

suite = suite.add("Decimal.toString", () => {
  decimal.toString();
});

suite = suite.add("Decimal.multiplyPow10", () => {
  decimal.multiplyPow10(2);
  decimal.multiplyPow10(-2);
});

suite = suite.add("Decimal.sign", () => {
  decimal.sign = DecimalSign.Negative;
  decimal.sign = new DecimalSign("Positive");
  decimal.sign = null;
});

suite
  .on('cycle', (event) => {
    console.log(String(event.target));
    console.log('μs/it:', event.target.stats.mean * 1000 * 1000);
    console.log();
  })
  .run({ "async": false });
