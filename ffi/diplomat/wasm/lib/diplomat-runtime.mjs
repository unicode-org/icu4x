function readString(wasm, ptr, len) {
  const buf = new Uint8Array(wasm.memory.buffer, ptr, len);
  return (new TextDecoder("utf-8")).decode(buf)
}

export function withWriteable(wasm, callback) {
  const writeable = wasm.diplomat_buffer_writeable_create(0);
  try {
    callback(writeable);
    const outStringPtr = wasm.diplomat_buffer_writeable_get_bytes(writeable);
    const outStringLen = wasm.diplomat_buffer_writeable_len(writeable);
    return readString(wasm, outStringPtr, outStringLen);
  } finally {
    wasm.diplomat_buffer_writeable_destroy(writeable);
  }
}

export class FFIError extends Error {
  constructor(error_value) {
    super("Error over FFI");
    this.error_value = error_value; // (2)
  }
}

export function extractCodePoint(str, param) {
  const cp = str.codePointAt?.(0);
  if ((!cp && cp !== 0) || [...str]?.length != 1) {
    throw new TypeError(`Expected single-character string for char parameter ${param}, found ${str}`);
  }
  return cp;
}
