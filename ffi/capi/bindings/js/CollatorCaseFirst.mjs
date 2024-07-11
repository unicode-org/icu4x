import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const CollatorCaseFirst_js_to_rust = {
  "Auto": 0,
  "Off": 1,
  "LowerFirst": 2,
  "UpperFirst": 3,
};

export const CollatorCaseFirst_rust_to_js = {
  [0]: "Auto",
  [1]: "Off",
  [2]: "LowerFirst",
  [3]: "UpperFirst",
};

export const CollatorCaseFirst = {
  "Auto": "Auto",
  "Off": "Off",
  "LowerFirst": "LowerFirst",
  "UpperFirst": "UpperFirst",
};
