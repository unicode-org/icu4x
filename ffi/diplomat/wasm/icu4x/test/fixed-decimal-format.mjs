// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import test from 'ava';

import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormatter } from "../lib/index.js"

const locale = ICU4XLocale.create("bn");
const dataProvider = ICU4XDataProvider.create_test();
const format = ICU4XFixedDecimalFormatter.try_new(dataProvider, locale, "Auto");

test("format a simple decimal", t => {
  const decimal = ICU4XFixedDecimal.create(1234);
  decimal.multiply_pow10(-2);

  t.is(format.format(decimal), "১২.৩৪");
});

test("format a long decimal", t => {
  const decimal = ICU4XFixedDecimal.create(1000007);

  t.is(format.format(decimal), "১০,০০,০০৭");
});

test("format a negated, scaled decimal", t => {
  const decimal = ICU4XFixedDecimal.create(1000007);
  decimal.multiply_pow10(2);
  decimal.set_sign("Negative");

  t.is(format.format(decimal), "-১০,০০,০০,৭০০");
});
