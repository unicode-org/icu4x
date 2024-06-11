import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const ICU4XDataError_js_to_rust = {
  "Unknown": 0,
  "MissingDataMarker": 1,
  "MissingVariant": 2,
  "MissingLocale": 3,
  "NeedsVariant": 4,
  "NeedsLocale": 5,
  "ExtraneousLocale": 6,
  "FilteredResource": 7,
  "MismatchedType": 8,
  "MissingPayload": 9,
  "InvalidState": 10,
  "Custom": 11,
  "Io": 12,
  "UnavailableBufferFormat": 13,
  "MismatchedAnyBuffer": 14,
  "DataStructValidityError": 15,
};

export const ICU4XDataError_rust_to_js = {
  [0]: "Unknown",
  [1]: "MissingDataMarker",
  [2]: "MissingVariant",
  [3]: "MissingLocale",
  [4]: "NeedsVariant",
  [5]: "NeedsLocale",
  [6]: "ExtraneousLocale",
  [7]: "FilteredResource",
  [8]: "MismatchedType",
  [9]: "MissingPayload",
  [10]: "InvalidState",
  [11]: "Custom",
  [12]: "Io",
  [13]: "UnavailableBufferFormat",
  [14]: "MismatchedAnyBuffer",
  [15]: "DataStructValidityError",
};

export const ICU4XDataError = {
  "Unknown": "Unknown",
  "MissingDataMarker": "MissingDataMarker",
  "MissingVariant": "MissingVariant",
  "MissingLocale": "MissingLocale",
  "NeedsVariant": "NeedsVariant",
  "NeedsLocale": "NeedsLocale",
  "ExtraneousLocale": "ExtraneousLocale",
  "FilteredResource": "FilteredResource",
  "MismatchedType": "MismatchedType",
  "MissingPayload": "MissingPayload",
  "InvalidState": "InvalidState",
  "Custom": "Custom",
  "Io": "Io",
  "UnavailableBufferFormat": "UnavailableBufferFormat",
  "MismatchedAnyBuffer": "MismatchedAnyBuffer",
  "DataStructValidityError": "DataStructValidityError",
};
