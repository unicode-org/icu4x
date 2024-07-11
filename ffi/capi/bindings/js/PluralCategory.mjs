import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const PluralCategory_js_to_rust = {
  "Zero": 0,
  "One": 1,
  "Two": 2,
  "Few": 3,
  "Many": 4,
  "Other": 5,
};

export const PluralCategory_rust_to_js = {
  [0]: "Zero",
  [1]: "One",
  [2]: "Two",
  [3]: "Few",
  [4]: "Many",
  [5]: "Other",
};

export const PluralCategory = {
  "Zero": "Zero",
  "One": "One",
  "Two": "Two",
  "Few": "Few",
  "Many": "Many",
  "Other": "Other",
};
