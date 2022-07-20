// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

let wasm;

function readString(ptr) {
  const view = new Uint8Array(wasm.memory.buffer);
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
  const paths = await import("./paths.mjs");
  const wasmFile = new Uint8Array(fs.readFileSync(paths.WASM_PATH));
  const loadedWasm = await WebAssembly.instantiate(wasmFile, imports);
  wasm = loadedWasm.instance.exports;
} else {
  const pathsToTry = [
    "./icu_capi.wasm",
    "../../../../wasmpkg/icu_capi.wasm",
    "./icu_capi_cdylib.wasm",
    "../../../../wasmpkg/icu_capi_cdylib.wasm",
  ];
  let loadedWasm;
  for (const path of pathsToTry) {
    try {
      loadedWasm = await WebAssembly.instantiateStreaming(fetch(path), imports);
      wasm = loadedWasm.instance.exports;
      break;
    } catch(e) {
    }
  }
  if (!loadedWasm) {
    console.error("Could not find icu_capi.wasm");
  }
}

wasm.diplomat_init();
wasm.icu4x_init();

export default wasm;
