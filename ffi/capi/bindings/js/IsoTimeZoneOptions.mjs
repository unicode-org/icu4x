import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { IsoTimeZoneFormat_js_to_rust, IsoTimeZoneFormat_rust_to_js } from "./IsoTimeZoneFormat.mjs"
import { IsoTimeZoneMinuteDisplay_js_to_rust, IsoTimeZoneMinuteDisplay_rust_to_js } from "./IsoTimeZoneMinuteDisplay.mjs"
import { IsoTimeZoneSecondDisplay_js_to_rust, IsoTimeZoneSecondDisplay_rust_to_js } from "./IsoTimeZoneSecondDisplay.mjs"

export class IsoTimeZoneOptions {
  constructor(underlying) {
    this.format = IsoTimeZoneFormat_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.minutes = IsoTimeZoneMinuteDisplay_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
    this.seconds = IsoTimeZoneSecondDisplay_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 8)];
  }
}
