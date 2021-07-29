function readString(wasm, ptr, len) {
  const buf = new Uint8Array(wasm.memory.buffer, ptr, len);
  return (new TextDecoder("utf-8")).decode(buf)
}

export function withWriteable(wasm, callback) {
  const writeable = wasm.diplomat_buffer_writeable_create(0);
  try {
    const callbackOut = callback(writeable);
    if (typeof callbackOut  === "object" && callbackOut.hasOwnProperty("is_ok")) {
      if (callbackOut.is_ok) {
        const outStringPtr = wasm.diplomat_buffer_writeable_get_bytes(writeable);
        const outStringLen = wasm.diplomat_buffer_writeable_len(writeable);
        return { is_ok: true, ok: readString(wasm, outStringPtr, outStringLen) };
      } else {
        return callbackOut;
      }
    } else {
      const outStringPtr = wasm.diplomat_buffer_writeable_get_bytes(writeable);
      const outStringLen = wasm.diplomat_buffer_writeable_len(writeable);
      return readString(wasm, outStringPtr, outStringLen);
    }
  } finally {
    wasm.diplomat_buffer_writeable_destroy(writeable);
  }
}
