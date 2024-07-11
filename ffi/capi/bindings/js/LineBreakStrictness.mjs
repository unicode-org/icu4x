import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const LineBreakStrictness_js_to_rust = {
  "Loose": 0,
  "Normal": 1,
  "Strict": 2,
  "Anywhere": 3,
};

export const LineBreakStrictness_rust_to_js = {
  [0]: "Loose",
  [1]: "Normal",
  [2]: "Strict",
  [3]: "Anywhere",
};

export const LineBreakStrictness = {
  "Loose": "Loose",
  "Normal": "Normal",
  "Strict": "Strict",
  "Anywhere": "Anywhere",
};
