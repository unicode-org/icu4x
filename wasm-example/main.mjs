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

const fixedDecimalRegistry = new FinalizationRegistry(ptr => {
  console.log("freeing decimal!");
  icu4x.icu4x_fixed_decimal_destroy(ptr);
});

class FixedDecimal {
  constructor(magnitude) {
    this.underlying = icu4x.icu4x_fixed_decimal_create(magnitude);    
    fixedDecimalRegistry.register(this, this.underlying);
  }

  multiply_pow10(pow) {
    icu4x.icu4x_fixed_decimal_multiply_pow10(this.underlying, pow);
  }

  negate() {
    icu4x.icu4x_fixed_decimal_negate(this.underlying);
  }

  write_to(writable) {
    icu4x.icu4x_fixed_decimal_write_to(this.underlying, writable.underlying);
  }
}

const bufferWritableRegistry = new FinalizationRegistry(ptr => {
  console.log("freeing writable!");
  icu4x.icu4x_buffer_writeable_free(ptr);
});

class BufferWritable {
  constructor() {
    this.underlying = icu4x.icu4x_buffer_writeable(0);    
    bufferWritableRegistry.register(this, this.underlying);
  }

  getString() {
    const outStringPtr = icu4x.icu4x_buffer_writeable_borrow(this.underlying);
    const outStringLen = icu4x.icu4x_buffer_writeable_len(this.underlying);
    return getString(outStringPtr, outStringLen);
  }
}

const decimal = new FixedDecimal(BigInt(1234));
decimal.multiply_pow10(-2);
decimal.negate();

const outWritable = new BufferWritable();
decimal.write_to(outWritable);
console.log(outWritable.getString());
