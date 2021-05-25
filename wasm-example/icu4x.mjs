let icu4x;

function readString(ptr) {
  const view = new Uint8Array(icu4x.memory.buffer);
  let end = ptr;
  while (view[end]) ++ end;
  const buf = new Uint8Array(view.subarray(ptr, end));
  return (new TextDecoder()).decode(buf);
}

const imports = {
  env: {
    log_js(ptr) {
      console.log(readString(ptr));
    },

    warn_js(ptr) {
      console.warn(readString(ptr));
    },

    trace_js(ptr) {
      throw new Error(readString(ptr));
    }
  }
}

const ixu4xWasm = await WebAssembly.instantiateStreaming(fetch('../wasmpkg/icu_capi.wasm'), imports);
icu4x = ixu4xWasm.instance.exports;

icu4x.icu4x_init();

export default icu4x;
