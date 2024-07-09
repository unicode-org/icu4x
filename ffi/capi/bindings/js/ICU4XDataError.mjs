import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const ICU4XDataError_js_to_rust = {
  "Unknown": 0,
  "MarkerNotFound": 1,
  "IdentifierNotFound": 2,
  "InvalidRequest": 3,
  "InconsistentData": 4,
  "Downcast": 5,
  "Deserialize": 6,
  "Custom": 7,
  "Io": 8,
};

export const ICU4XDataError_rust_to_js = {
  [0]: "Unknown",
  [1]: "MarkerNotFound",
  [2]: "IdentifierNotFound",
  [3]: "InvalidRequest",
  [4]: "InconsistentData",
  [5]: "Downcast",
  [6]: "Deserialize",
  [7]: "Custom",
  [8]: "Io",
};

export const ICU4XDataError = {
  "Unknown": "Unknown",
  "MarkerNotFound": "MarkerNotFound",
  "IdentifierNotFound": "IdentifierNotFound",
  "InvalidRequest": "InvalidRequest",
  "InconsistentData": "InconsistentData",
  "Downcast": "Downcast",
  "Deserialize": "Deserialize",
  "Custom": "Custom",
  "Io": "Io",
};
