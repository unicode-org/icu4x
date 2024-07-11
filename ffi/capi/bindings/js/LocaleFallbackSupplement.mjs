import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const LocaleFallbackSupplement_js_to_rust = {
  "None": 0,
  "Collation": 1,
};

export const LocaleFallbackSupplement_rust_to_js = {
  [0]: "None",
  [1]: "Collation",
};

export const LocaleFallbackSupplement = {
  "None": "None",
  "Collation": "Collation",
};
