import Benchmark from 'benchmark';

import { Decimal, Locale, DecimalFormatter, DecimalGroupingStrategy } from "icu4x"

const locale = Locale.fromString("bn");

let suite = new Benchmark.Suite();

suite = suite.add("DecimalFormatter.create", () => {
  DecimalFormatter.createWithGroupingStrategy(locale, DecimalGroupingStrategy.Auto);
});

const format = DecimalFormatter.createWithGroupingStrategy(locale, DecimalGroupingStrategy.Auto);
const decimal = Decimal.fromBigInt(BigInt(1234));
decimal.multiplyPow10(-2);

suite = suite.add("DecimalFormatter.format", () => {
  format.format(decimal);
});

suite
  .on('cycle', (event) => {
    console.log(String(event.target));
    console.log('μs/it:', event.target.stats.mean * 1000 * 1000);
    console.log();
  })
  .run({ "async": false });
