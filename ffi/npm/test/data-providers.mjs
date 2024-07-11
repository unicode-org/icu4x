// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import test from 'ava';

import { FixedDecimal, Locale, DataProvider, FixedDecimalFormatter } from "icu4x"

test("use create_compiled to format a simple decimal", async t => {
  const locale = Locale.create_from_string("bn");
  const provider = DataProvider.create_compiled();

  const format = FixedDecimalFormatter.create_with_grouping_strategy(provider, locale, "Auto");

  const decimal = FixedDecimal.create_from_i32(1234);
  decimal.multiply_pow10(-2);

  t.is(format.format(decimal), "১২.৩৪");
});
