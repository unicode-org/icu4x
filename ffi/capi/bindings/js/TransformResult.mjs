import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const TransformResult_js_to_rust = {
  "Modified": 0,
  "Unmodified": 1,
};

export const TransformResult_rust_to_js = {
  [0]: "Modified",
  [1]: "Unmodified",
};

export const TransformResult = {
  "Modified": "Modified",
  "Unmodified": "Unmodified",
};
