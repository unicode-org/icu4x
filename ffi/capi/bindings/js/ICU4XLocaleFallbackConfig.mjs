import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XLocaleFallbackPriority_js_to_rust, ICU4XLocaleFallbackPriority_rust_to_js } from "./ICU4XLocaleFallbackPriority.mjs"
import { ICU4XLocaleFallbackSupplement_js_to_rust, ICU4XLocaleFallbackSupplement_rust_to_js } from "./ICU4XLocaleFallbackSupplement.mjs"

export class ICU4XLocaleFallbackConfig {
  constructor(underlying) {
    this.priority = ICU4XLocaleFallbackPriority_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.fallback_supplement = ICU4XLocaleFallbackSupplement_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
  }
}
