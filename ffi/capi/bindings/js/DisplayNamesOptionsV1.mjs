import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DisplayNamesFallback_js_to_rust, DisplayNamesFallback_rust_to_js } from "./DisplayNamesFallback.mjs"
import { DisplayNamesStyle_js_to_rust, DisplayNamesStyle_rust_to_js } from "./DisplayNamesStyle.mjs"
import { LanguageDisplay_js_to_rust, LanguageDisplay_rust_to_js } from "./LanguageDisplay.mjs"

export class DisplayNamesOptionsV1 {
  constructor(underlying) {
    this.style = DisplayNamesStyle_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.fallback = DisplayNamesFallback_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
    this.language_display = LanguageDisplay_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 8)];
  }
}
