import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { LocaleFallbackIterator } from "./LocaleFallbackIterator.mjs"

const LocaleFallbackerWithConfig_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLocaleFallbackerWithConfig_destroy(underlying);
});

export class LocaleFallbackerWithConfig {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      LocaleFallbackerWithConfig_box_destroy_registry.register(this, underlying);
    }
  }

  fallback_for_locale(arg_locale) {
    return new LocaleFallbackIterator(wasm.ICU4XLocaleFallbackerWithConfig_fallback_for_locale(this.underlying, arg_locale.underlying), true, [this]);
  }
}
