// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import test from 'ava';

import { Decimal, DecimalSign } from 'icu';

test('convert a simple decimal to a string', t => {
  const decimal = Decimal.fromBigInt(1234n);

  t.is(decimal.toString(), '1234');
});

test('multiply a decimal by a power of 10', t => {
  const decimal = Decimal.fromNumber(1234);
  decimal.multiplyPow10(-2);

  t.is(decimal.toString(), '12.34');
});

test('negate a decimal', t => {
  const decimal = Decimal.fromNumber(1234);
  decimal.sign = DecimalSign.Negative;

  t.is(decimal.toString(), '-1234');
});
