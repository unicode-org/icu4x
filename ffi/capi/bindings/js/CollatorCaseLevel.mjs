import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const CollatorCaseLevel_js_to_rust = {
  "Auto": 0,
  "Off": 1,
  "On": 2,
};

export const CollatorCaseLevel_rust_to_js = {
  [0]: "Auto",
  [1]: "Off",
  [2]: "On",
};

export const CollatorCaseLevel = {
  "Auto": "Auto",
  "Off": "Off",
  "On": "On",
};
