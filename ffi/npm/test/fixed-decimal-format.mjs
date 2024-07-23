// This file is part of . For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import test from 'ava';

import { FixedDecimal, Locale, DataProvider, FixedDecimalFormatter, FixedDecimalSign, FixedDecimalGroupingStrategy } from 'icu4x';

const locale = Locale.fromString('bn');
const provider = DataProvider.compiled();
const format = FixedDecimalFormatter.createWithGroupingStrategy(provider, locale, FixedDecimalGroupingStrategy.Auto);

test('format a simple decimal', t => {
  const decimal = FixedDecimal.fromNumber(1234);
  decimal.multiplyPow10(-2);

  t.is(format.format(decimal), '১২.৩৪');
});

test('format a long decimal', t => {
  const decimal = FixedDecimal.fromNumber(1000007);

  t.is(format.format(decimal), '১০,০০,০০৭');
});

test('format a negated, scaled decimal', t => {
  const decimal = FixedDecimal.fromNumber(1000007);
  decimal.multiplyPow10(2);
  decimal.sign = FixedDecimalSign.Negative;

  t.is(format.format(decimal), '-১০,০০,০০,৭০০');
});
