import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const DisplayNamesStyle_js_to_rust = {
  "Auto": 0,
  "Narrow": 1,
  "Short": 2,
  "Long": 3,
  "Menu": 4,
};

export const DisplayNamesStyle_rust_to_js = {
  [0]: "Auto",
  [1]: "Narrow",
  [2]: "Short",
  [3]: "Long",
  [4]: "Menu",
};

export const DisplayNamesStyle = {
  "Auto": "Auto",
  "Narrow": "Narrow",
  "Short": "Short",
  "Long": "Long",
  "Menu": "Menu",
};
