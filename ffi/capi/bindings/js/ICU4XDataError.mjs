import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const ICU4XDataError_js_to_rust = {
  "Unknown": 0,
  "MissingDataMarker": 1,
  "MissingLocale": 2,
  "NeedsLocale": 3,
  "ExtraneousLocale": 4,
  "FilteredResource": 5,
  "MismatchedType": 6,
  "Custom": 7,
  "Io": 8,
  "UnavailableBufferFormat": 9,
  "InconsistentData": 10,
};

export const ICU4XDataError_rust_to_js = {
  [0]: "Unknown",
  [1]: "MissingDataMarker",
  [2]: "MissingLocale",
  [3]: "NeedsLocale",
  [4]: "ExtraneousLocale",
  [5]: "FilteredResource",
  [6]: "MismatchedType",
  [7]: "Custom",
  [8]: "Io",
  [9]: "UnavailableBufferFormat",
  [10]: "InconsistentData",
};

export const ICU4XDataError = {
  "Unknown": "Unknown",
  "MissingDataMarker": "MissingDataMarker",
  "MissingLocale": "MissingLocale",
  "NeedsLocale": "NeedsLocale",
  "ExtraneousLocale": "ExtraneousLocale",
  "FilteredResource": "FilteredResource",
  "MismatchedType": "MismatchedType",
  "Custom": "Custom",
  "Io": "Io",
  "UnavailableBufferFormat": "UnavailableBufferFormat",
  "InconsistentData": "InconsistentData",
};
