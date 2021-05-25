import icu4x from "./icu4x.mjs"

function withEncodedString(str, fn) {
  let bytes = (new TextEncoder()).encode(str);

  let ptr = icu4x.icu4x_alloc(bytes.length);
  const memory = new Uint8Array(icu4x.memory.buffer);
  const buf = memory.subarray(ptr, ptr + bytes.length);
  buf.set(bytes, 0);

  try {
    return fn(ptr, buf.length);
  } finally {
    icu4x.icu4x_free(ptr, buf.length);
  }
}

function getString(ptr, len) {
  const memory = new Uint8Array(icu4x.memory.buffer);
  const buf = memory.subarray(ptr, ptr + len);
  return (new TextDecoder()).decode(buf)
}

const decimal = icu4x.icu4x_fixed_decimal_create(BigInt(1234));
icu4x.icu4x_fixed_decimal_multiply_pow10(decimal, -2);
icu4x.icu4x_fixed_decimal_negate(decimal);

const outWritable = icu4x.icu4x_buffer_writeable(10);
icu4x.icu4x_fixed_decimal_write_to(decimal, outWritable);
const outStringPtr = icu4x.icu4x_buffer_writeable_borrow(outWritable);
const outStringLen = icu4x.icu4x_buffer_writeable_len(outWritable);
console.log(getString(outStringPtr, outStringLen));
icu4x.icu4x_buffer_writeable_free(outWritable);
icu4x.icu4x_fixed_decimal_destroy(decimal);
