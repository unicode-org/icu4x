import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const FixedDecimalGroupingStrategy_js_to_rust = {
  "Auto": 0,
  "Never": 1,
  "Always": 2,
  "Min2": 3,
};

export const FixedDecimalGroupingStrategy_rust_to_js = {
  [0]: "Auto",
  [1]: "Never",
  [2]: "Always",
  [3]: "Min2",
};

export const FixedDecimalGroupingStrategy = {
  "Auto": "Auto",
  "Never": "Never",
  "Always": "Always",
  "Min2": "Min2",
};
