import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const CollatorAlternateHandling_js_to_rust = {
  "Auto": 0,
  "NonIgnorable": 1,
  "Shifted": 2,
};

export const CollatorAlternateHandling_rust_to_js = {
  [0]: "Auto",
  [1]: "NonIgnorable",
  [2]: "Shifted",
};

export const CollatorAlternateHandling = {
  "Auto": "Auto",
  "NonIgnorable": "NonIgnorable",
  "Shifted": "Shifted",
};
