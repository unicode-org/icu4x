// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import Benchmark from 'benchmark';

import { Decimal, Locale, DecimalFormatter, DecimalGroupingStrategy } from 'icu'

const locale = Locale.fromString('bn');

let suite = new Benchmark.Suite();

suite = suite.add('DecimalFormatter.create', () => {
  DecimalFormatter.createWithGroupingStrategy(locale, DecimalGroupingStrategy.Auto);
});

const format = DecimalFormatter.createWithGroupingStrategy(locale, DecimalGroupingStrategy.Auto);
const decimal = Decimal.fromBigInt(BigInt(1234));
decimal.multiplyPow10(-2);

suite = suite.add('DecimalFormatter.format', () => {
  format.format(decimal);
});

suite
  .on('cycle', (event) => {
    console.log(String(event.target));
    console.log('μs/it:', event.target.stats.mean * 1000 * 1000);
    console.log();
  })
  .run({ 'async': false });
