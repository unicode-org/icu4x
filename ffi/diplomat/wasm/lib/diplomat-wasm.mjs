import cfg from '../diplomat.config.js';

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

if (typeof fetch === 'undefined') { // Node
  const fs = await import("fs");
  const wasmFile = new Uint8Array(fs.readFileSync(cfg['wasm_path']));
  const loadedWasm = await WebAssembly.instantiate(wasmFile, imports);
  wasm = loadedWasm.instance.exports;
} else { // Browser
  const loadedWasm = await WebAssembly.instantiateStreaming(fetch(cfg['wasm_path']), imports);
  wasm = loadedWasm.instance.exports;
}

wasm.diplomat_init();
if (cfg['init'] !== undefined) {
  cfg['init'](wasm);
}

export default wasm;
