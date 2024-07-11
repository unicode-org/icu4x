import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const DisplayNamesFallback_js_to_rust = {
  "Code": 0,
  "None": 1,
};

export const DisplayNamesFallback_rust_to_js = {
  [0]: "Code",
  [1]: "None",
};

export const DisplayNamesFallback = {
  "Code": "Code",
  "None": "None",
};
