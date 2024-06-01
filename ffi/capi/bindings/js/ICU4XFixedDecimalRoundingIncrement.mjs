import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const ICU4XFixedDecimalRoundingIncrement_js_to_rust = {
  "MultiplesOf1": 0,
  "MultiplesOf2": 1,
  "MultiplesOf5": 2,
  "MultiplesOf25": 3,
};

export const ICU4XFixedDecimalRoundingIncrement_rust_to_js = {
  [0]: "MultiplesOf1",
  [1]: "MultiplesOf2",
  [2]: "MultiplesOf5",
  [3]: "MultiplesOf25",
};

export const ICU4XFixedDecimalRoundingIncrement = {
  "MultiplesOf1": "MultiplesOf1",
  "MultiplesOf2": "MultiplesOf2",
  "MultiplesOf5": "MultiplesOf5",
  "MultiplesOf25": "MultiplesOf25",
};
