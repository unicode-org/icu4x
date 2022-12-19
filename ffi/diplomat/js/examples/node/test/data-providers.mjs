// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import test from 'ava';

import { ICU4XFixedDecimal, ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormatter, fullData } from "../index.js"

test("use create_from_byte_slice to format a simple decimal", async t => {
  const locale = ICU4XLocale.create_from_string("bn");
  const provider = ICU4XDataProvider.create_from_byte_slice(await fullData());

  const format = ICU4XFixedDecimalFormatter.create_with_grouping_strategy(provider, locale, "Auto");

  const decimal = ICU4XFixedDecimal.create_from_i32(1234);
  decimal.multiply_pow10(-2);

  t.is(format.format(decimal), "১২.৩৪");
});

test("fail to create from invalid buffer", t => {
  const arrayBuffer = new ArrayBuffer(8);
  const bytes = new Uint8Array(arrayBuffer);
  // Fill the buffer with junk data
  for (let i = 0; i < bytes.length; i++) {
    bytes[i] = i;
  }
  t.throws(() => {
    const result = ICU4XDataProvider.create_from_byte_slice(bytes);
  });
  
});
