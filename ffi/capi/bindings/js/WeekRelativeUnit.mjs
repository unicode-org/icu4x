import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const WeekRelativeUnit_js_to_rust = {
  "Previous": 0,
  "Current": 1,
  "Next": 2,
};

export const WeekRelativeUnit_rust_to_js = {
  [0]: "Previous",
  [1]: "Current",
  [2]: "Next",
};

export const WeekRelativeUnit = {
  "Previous": "Previous",
  "Current": "Current",
  "Next": "Next",
};
