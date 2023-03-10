import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XFallback_js_to_rust, ICU4XFallback_rust_to_js } from "./ICU4XFallback.js"
import { ICU4XLanguageDisplay_js_to_rust, ICU4XLanguageDisplay_rust_to_js } from "./ICU4XLanguageDisplay.js"
import { ICU4XStyle_js_to_rust, ICU4XStyle_rust_to_js } from "./ICU4XStyle.js"

export class ICU4XDisplayNamesOptions {
  constructor(underlying) {
    this.style = ICU4XStyle_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.fallback = ICU4XFallback_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
    this.language_display = ICU4XLanguageDisplay_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 8)];
  }
}
