import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

export const ICU4XLeadingAdjustment_js_to_rust = {
  "None": 0,
  "Auto": 1,
  "AdjustToCased": 2,
};

export const ICU4XLeadingAdjustment_rust_to_js = {
  [0]: "None",
  [1]: "Auto",
  [2]: "AdjustToCased",
};

export const ICU4XLeadingAdjustment = {
  "None": "None",
  "Auto": "Auto",
  "AdjustToCased": "AdjustToCased",
};
