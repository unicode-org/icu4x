import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const ICU4XCalendarError_js_to_rust = {
  "Unknown": 0,
  "OutOfRange": 1,
  "UnknownEra": 2,
  "UnknownMonthCode": 3,
};

export const ICU4XCalendarError_rust_to_js = {
  [0]: "Unknown",
  [1]: "OutOfRange",
  [2]: "UnknownEra",
  [3]: "UnknownMonthCode",
};

export const ICU4XCalendarError = {
  "Unknown": "Unknown",
  "OutOfRange": "OutOfRange",
  "UnknownEra": "UnknownEra",
  "UnknownMonthCode": "UnknownMonthCode",
};
