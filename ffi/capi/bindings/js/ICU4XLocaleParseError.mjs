import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const ICU4XLocaleParseError_js_to_rust = {
  "Unknown": 0,
  "Language": 1,
  "Subtag": 2,
  "Extension": 3,
};

export const ICU4XLocaleParseError_rust_to_js = {
  [0]: "Unknown",
  [1]: "Language",
  [2]: "Subtag",
  [3]: "Extension",
};

export const ICU4XLocaleParseError = {
  "Unknown": "Unknown",
  "Language": "Language",
  "Subtag": "Subtag",
  "Extension": "Extension",
};
