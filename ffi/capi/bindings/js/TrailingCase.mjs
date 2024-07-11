import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const TrailingCase_js_to_rust = {
  "Lower": 0,
  "Unchanged": 1,
};

export const TrailingCase_rust_to_js = {
  [0]: "Lower",
  [1]: "Unchanged",
};

export const TrailingCase = {
  "Lower": "Lower",
  "Unchanged": "Unchanged",
};
