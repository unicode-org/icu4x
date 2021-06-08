import { FixedDecimal, BufferWritable } from "../lib/high-level.mjs"

const decimal = new FixedDecimal(BigInt(1234));
decimal.multiply_pow10(-2);
decimal.negate();

const outWritable = new BufferWritable();
decimal.write_to(outWritable);
console.log(outWritable.getString());
