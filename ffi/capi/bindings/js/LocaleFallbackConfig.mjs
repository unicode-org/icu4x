import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { LocaleFallbackPriority_js_to_rust, LocaleFallbackPriority_rust_to_js } from "./LocaleFallbackPriority.mjs"
import { LocaleFallbackSupplement_js_to_rust, LocaleFallbackSupplement_rust_to_js } from "./LocaleFallbackSupplement.mjs"

export class LocaleFallbackConfig {
  constructor(underlying, edges_a) {
    this.priority = LocaleFallbackPriority_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.extension_key = (() => {
      const [ptr, size] = new Uint32Array(wasm.memory.buffer, underlying + 4, 2);
      return diplomatRuntime.readString8(wasm, ptr, size);
    })();
    this.fallback_supplement = LocaleFallbackSupplement_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 12)];
  }
}
