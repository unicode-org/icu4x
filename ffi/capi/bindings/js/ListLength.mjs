import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const ListLength_js_to_rust = {
  "Wide": 0,
  "Short": 1,
  "Narrow": 2,
};

export const ListLength_rust_to_js = {
  [0]: "Wide",
  [1]: "Short",
  [2]: "Narrow",
};

export const ListLength = {
  "Wide": "Wide",
  "Short": "Short",
  "Narrow": "Narrow",
};
