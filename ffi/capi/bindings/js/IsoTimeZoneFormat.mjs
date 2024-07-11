import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

export const IsoTimeZoneFormat_js_to_rust = {
  "Basic": 0,
  "Extended": 1,
  "UtcBasic": 2,
  "UtcExtended": 3,
};

export const IsoTimeZoneFormat_rust_to_js = {
  [0]: "Basic",
  [1]: "Extended",
  [2]: "UtcBasic",
  [3]: "UtcExtended",
};

export const IsoTimeZoneFormat = {
  "Basic": "Basic",
  "Extended": "Extended",
  "UtcBasic": "UtcBasic",
  "UtcExtended": "UtcExtended",
};
