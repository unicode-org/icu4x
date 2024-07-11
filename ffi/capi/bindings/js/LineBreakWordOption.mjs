import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const LineBreakWordOption_js_to_rust = {
  "Normal": 0,
  "BreakAll": 1,
  "KeepAll": 2,
};

export const LineBreakWordOption_rust_to_js = {
  [0]: "Normal",
  [1]: "BreakAll",
  [2]: "KeepAll",
};

export const LineBreakWordOption = {
  "Normal": "Normal",
  "BreakAll": "BreakAll",
  "KeepAll": "KeepAll",
};
