import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

export const ICU4XOrdering_js_to_rust = {
  "Less": 0,
  "Equal": 1,
  "Greater": 2,
};

export const ICU4XOrdering_rust_to_js = {
  0: "Less",
  1: "Equal",
  2: "Greater",
};

export const ICU4XOrdering = {
  "Less": "Less",
  "Equal": "Equal",
  "Greater": "Greater",
};
