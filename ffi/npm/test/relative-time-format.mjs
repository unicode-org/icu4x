// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import test from 'ava';

import {
  Decimal,
  DecimalSign,
  Locale,
  RelativeTimeFormatter,
  RelativeTimeNumeric,
} from 'icu';

test('format relative time', t => {
  const locale = Locale.fromString('en');
  const formatter = RelativeTimeFormatter.createLongDay(locale, {
    numeric: RelativeTimeNumeric.Auto,
  });

  // 1. Numeric::Auto relative literal (-1 -> "yesterday")
  const minusOne = Decimal.fromNumber(-1);
  t.is(formatter.format(minusOne), 'yesterday');

  // 2. Numeric::Auto with -0 -> "0 days ago"
  const negZero = Decimal.fromNumber(0);
  negZero.sign = DecimalSign.Negative;
  t.is(formatter.format(negZero), '0 days ago');

  // 3. Grouped number (1234 -> "in 1,234 days")
  const thousand = Decimal.fromNumber(1234);
  t.is(formatter.format(thousand), 'in 1,234 days');

  // 4. Bengali ("bn", narrow year)
  const bn = Locale.fromString('bn');
  const bnFormatter = RelativeTimeFormatter.createNarrowYear(bn, {});
  const three = Decimal.fromNumber(3);
  t.is(bnFormatter.format(three), '৩ বছরে');
});
