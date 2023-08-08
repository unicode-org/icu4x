import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

export const ICU4XHeadAdjustment_js_to_rust = {
  "Adjust": 0,
  "NoAdjust": 1,
};

export const ICU4XHeadAdjustment_rust_to_js = {
  [0]: "Adjust",
  [1]: "NoAdjust",
};

export const ICU4XHeadAdjustment = {
  "Adjust": "Adjust",
  "NoAdjust": "NoAdjust",
};
