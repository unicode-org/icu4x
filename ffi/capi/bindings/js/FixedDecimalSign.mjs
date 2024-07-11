import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const FixedDecimalSign_js_to_rust = {
  "None": 0,
  "Negative": 1,
  "Positive": 2,
};

export const FixedDecimalSign_rust_to_js = {
  [0]: "None",
  [1]: "Negative",
  [2]: "Positive",
};

export const FixedDecimalSign = {
  "None": "None",
  "Negative": "Negative",
  "Positive": "Positive",
};
