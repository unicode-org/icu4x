import Benchmark from 'benchmark';

import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDecimalFormatter, ICU4XDataProvider, ICU4XDecimalGroupingStrategy } from "icu4x"

const locale = ICU4XLocale.create_from_string("bn");
const dataProvider = ICU4XDataProvider.create_compiled();

let suite = new Benchmark.Suite();

suite = suite.add("ICU4XDecimalFormatter.create", () => {
  (ICU4XDecimalFormatter.create(locale, dataProvider, {})).underlying > 0;
});

const format = ICU4XDecimalFormatter.create_with_grouping_strategy(dataProvider, locale, ICU4XDecimalGroupingStrategy.Auto);
const decimal = ICU4XFixedDecimal.create_from_i64(BigInt(1234));
decimal.multiply_pow10(-2);

suite = suite.add("ICU4XDecimalFormatter.format", () => {
  format.format(decimal);
});

suite
  .on('cycle', (event) => {
    console.log(String(event.target));
    console.log('μs/it:', event.target.stats.mean * 1000 * 1000);
    console.log();
  })
  .run({ "async": false });
