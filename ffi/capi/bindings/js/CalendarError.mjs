import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const CalendarError_js_to_rust = {
  "Unknown": 0,
  "OutOfRange": 1,
  "UnknownEra": 2,
  "UnknownMonthCode": 3,
};

export const CalendarError_rust_to_js = {
  [0]: "Unknown",
  [1]: "OutOfRange",
  [2]: "UnknownEra",
  [3]: "UnknownMonthCode",
};

export const CalendarError = {
  "Unknown": "Unknown",
  "OutOfRange": "OutOfRange",
  "UnknownEra": "UnknownEra",
  "UnknownMonthCode": "UnknownMonthCode",
};
