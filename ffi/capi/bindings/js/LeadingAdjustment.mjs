import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const LeadingAdjustment_js_to_rust = {
  "Auto": 0,
  "None": 1,
  "ToCased": 2,
};

export const LeadingAdjustment_rust_to_js = {
  [0]: "Auto",
  [1]: "None",
  [2]: "ToCased",
};

export const LeadingAdjustment = {
  "Auto": "Auto",
  "None": "None",
  "ToCased": "ToCased",
};
