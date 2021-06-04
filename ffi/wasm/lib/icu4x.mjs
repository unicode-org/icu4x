let icu4x;

function readString(ptr) {
  const view = new Uint8Array(icu4x.memory.buffer);
  let end = ptr;
  while (view[end]) end++;
  return (new TextDecoder("utf-8")).decode(view.subarray(ptr, end));
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

if (typeof fetch === 'undefined') {
  const fs = await import("fs");
  const path = await import("path");
  const wasmFile = new Uint8Array(fs.readFileSync(path.resolve('../../../wasmpkg/icu_capi.wasm')));
  const ixu4xWasm = await WebAssembly.instantiate(wasmFile, imports);
  icu4x = ixu4xWasm.instance.exports;
} else {
  const ixu4xWasm = await WebAssembly.instantiateStreaming(fetch('../../../wasmpkg/icu_capi.wasm'), imports);
  icu4x = ixu4xWasm.instance.exports;
}

icu4x.icu4x_init();

export default icu4x;
