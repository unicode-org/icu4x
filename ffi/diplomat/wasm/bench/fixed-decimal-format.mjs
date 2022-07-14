import Benchmark from 'benchmark';

import { FixedDecimal, Locale, StaticDataProvider, FixedDecimalFormatter } from "../lib/high-level.mjs"

const locale = new Locale("bn");
const dataProvider = new StaticDataProvider();

let suite = new Benchmark.Suite();

suite = suite.add("new FixedDecimalFormatter", () => {
  (new FixedDecimalFormatter(locale, dataProvider, {})).underlying > 0;
});

const format = new FixedDecimalFormatter(locale, dataProvider, {});
const decimal = new FixedDecimal(BigInt(1234));
decimal.multiply_pow10(-2);

suite = suite.add("FixedDecimalFormatter.format", () => {
  format.format(decimal);
});

suite
  .on('cycle', (event) => {
    console.log(String(event.target));
    console.log('μs/it:', event.target.stats.mean * 1000 * 1000);
    console.log();
  })
  .run({ "async": false });
