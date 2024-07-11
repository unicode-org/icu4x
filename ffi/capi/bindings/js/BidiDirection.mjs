import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const BidiDirection_js_to_rust = {
  "Ltr": 0,
  "Rtl": 1,
  "Mixed": 2,
};

export const BidiDirection_rust_to_js = {
  [0]: "Ltr",
  [1]: "Rtl",
  [2]: "Mixed",
};

export const BidiDirection = {
  "Ltr": "Ltr",
  "Rtl": "Rtl",
  "Mixed": "Mixed",
};
