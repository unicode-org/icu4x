import Benchmark from 'benchmark';

import { FixedDecimal } from "../lib/high-level.mjs"

let suite = new Benchmark.Suite();

suite = suite.add("new FixedDecimal", () => {
  (new FixedDecimal(BigInt(1234))).underlying > 0;
});

const decimal = new FixedDecimal(BigInt(1234));
decimal.multiply_pow10(-2);

suite = suite.add("FixedDecimal.toString", () => {
  decimal.toString();
});

suite = suite.add("FixedDecimal.multiply_pow10", () => {
  decimal.multiply_pow10(2);
  decimal.multiply_pow10(-2);
});

suite = suite.add("FixedDecimal.set_sign", () => {
  decimal.set_sign("Negative");
  decimal.set_sign("None");
});

suite
  .on('cycle', (event) => {
    console.log(String(event.target));
    console.log('μs/it:', event.target.stats.mean * 1000 * 1000);
    console.log();
  })
  .run({ "async": false });
