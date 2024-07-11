import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const LanguageDisplay_js_to_rust = {
  "Dialect": 0,
  "Standard": 1,
};

export const LanguageDisplay_rust_to_js = {
  [0]: "Dialect",
  [1]: "Standard",
};

export const LanguageDisplay = {
  "Dialect": "Dialect",
  "Standard": "Standard",
};
