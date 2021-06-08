import test from 'ava';

import { FixedDecimal, BufferWritable } from "../lib/high-level.mjs"

test("convert a simple decimal to a string", t => {
  const decimal = new FixedDecimal(BigInt(1234));

  const outWritable = new BufferWritable();
  decimal.write_to(outWritable);
  t.is(outWritable.getString(), "1234");
});

test("multiply a decimal by a power of 10", t => {
  const decimal = new FixedDecimal(BigInt(1234));
  decimal.multiply_pow10(-2);

  const outWritable = new BufferWritable();
  decimal.write_to(outWritable);
  t.is(outWritable.getString(), "12.34");
});

test("negate a decimal", t => {
  const decimal = new FixedDecimal(BigInt(1234));
  decimal.negate();

  const outWritable = new BufferWritable();
  decimal.write_to(outWritable);
  t.is(outWritable.getString(), "-1234");
});
