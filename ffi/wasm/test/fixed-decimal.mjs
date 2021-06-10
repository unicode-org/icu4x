import test from 'ava';

import { FixedDecimal } from "../lib/high-level.mjs"

test("convert a simple decimal to a string", t => {
  const decimal = new FixedDecimal(BigInt(1234));

  t.is(decimal.toString(), "1234");
});

test("multiply a decimal by a power of 10", t => {
  const decimal = new FixedDecimal(BigInt(1234));
  decimal.multiply_pow10(-2);

  t.is(decimal.toString(), "12.34");
});

test("negate a decimal", t => {
  const decimal = new FixedDecimal(BigInt(1234));
  decimal.negate();

  t.is(decimal.toString(), "-1234");
});
