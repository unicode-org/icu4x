import { FixedDecimal, BufferWritable } from "../../wasm-lib/high-level.mjs"

const decimal = new FixedDecimal(BigInt(1234));
console.log(decimal.multiply_pow10(-2));
decimal.negate();

const outWritable = new BufferWritable();
decimal.write_to(outWritable);
console.log(outWritable.getString());
