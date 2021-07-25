import test from 'ava';

import { ICU4XFixedDecimal } from "../lib/api.mjs"

test("convert a simple decimal to a string", t => {
  const decimal = ICU4XFixedDecimal.create(1234);

  t.is(decimal.to_string(), "1234");
});

test("multiply a decimal by a power of 10", t => {
  const decimal = ICU4XFixedDecimal.create(1234);
  decimal.multiply_pow10(-2);

  t.is(decimal.to_string(), "12.34");
});

test("negate a decimal", t => {
  const decimal = ICU4XFixedDecimal.create(1234);
  decimal.negate();

  t.is(decimal.to_string(), "-1234");
});
