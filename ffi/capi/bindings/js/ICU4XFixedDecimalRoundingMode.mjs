import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const ICU4XFixedDecimalRoundingMode_js_to_rust = {
  "Ceil": 0,
  "Expand": 1,
  "Floor": 2,
  "Trunc": 3,
  "HalfCeil": 4,
  "HalfExpand": 5,
  "HalfFloor": 6,
  "HalfTrunc": 7,
  "HalfEven": 8,
};

export const ICU4XFixedDecimalRoundingMode_rust_to_js = {
  [0]: "Ceil",
  [1]: "Expand",
  [2]: "Floor",
  [3]: "Trunc",
  [4]: "HalfCeil",
  [5]: "HalfExpand",
  [6]: "HalfFloor",
  [7]: "HalfTrunc",
  [8]: "HalfEven",
};

export const ICU4XFixedDecimalRoundingMode = {
  "Ceil": "Ceil",
  "Expand": "Expand",
  "Floor": "Floor",
  "Trunc": "Trunc",
  "HalfCeil": "HalfCeil",
  "HalfExpand": "HalfExpand",
  "HalfFloor": "HalfFloor",
  "HalfTrunc": "HalfTrunc",
  "HalfEven": "HalfEven",
};
