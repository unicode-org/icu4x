// A collection of parsers for common scalar types.
export const ScalarType = {
  bool: (buffer, ptr) => {
    const val = (new Uint8Array(buffer, ptr, 1))[0]
    if (val == 0) {
      return false;
    } else if (val == 1) {
      return true;
    } else {
      throw Error(`Unexpected boolean value: ${val}`);
    }
  },
  i8: (buffer, ptr) => (new Int8Array(buffer, ptr, 1))[0],
  u8: (buffer, ptr) => (new Uint8Array(buffer, ptr, 1))[0],
  i16: (buffer, ptr) => (new Int16Array(buffer, ptr, 1))[0],
  u16: (buffer, ptr) => (new Uint16Array(buffer, ptr, 1))[0],
  i32: (buffer, ptr) => (new Int32Array(buffer, ptr, 1))[0],
  u32: (buffer, ptr) => (new Uint32Array(buffer, ptr, 1))[0],
  i64: (buffer, ptr) => (new BigInt64Array(buffer, ptr, 1))[0],
  u64: (buffer, ptr) => (new BigUint64Array(buffer, ptr, 1))[0]
}

// Generates a parser for a struct type.
// The struct shape is passed in as a dictionary, where the keys correspond
// to struct element names and the values are tuples of the offset in the
// struct and a parser for the value at that offset.
export function StructType(shape) {
  return (buffer, ptr) => {
    let out = {};
    Object.entries(shape).forEach(tup => {
      out[tup[0]] = tup[1][1](buffer, ptr + tup[1][0]);
    });
    return out;
  }
}
