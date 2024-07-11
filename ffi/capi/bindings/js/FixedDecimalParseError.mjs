import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const FixedDecimalParseError_js_to_rust = {
  "Unknown": 0,
  "Limit": 1,
  "Syntax": 2,
};

export const FixedDecimalParseError_rust_to_js = {
  [0]: "Unknown",
  [1]: "Limit",
  [2]: "Syntax",
};

export const FixedDecimalParseError = {
  "Unknown": "Unknown",
  "Limit": "Limit",
  "Syntax": "Syntax",
};
