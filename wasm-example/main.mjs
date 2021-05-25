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

const locale = withEncodedString("en-US", icu4x.icu4x_locale_create);
console.log(locale);
