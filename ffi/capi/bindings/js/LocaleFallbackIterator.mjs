import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { Locale } from "./Locale.mjs"

const LocaleFallbackIterator_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLocaleFallbackIterator_destroy(underlying);
});

export class LocaleFallbackIterator {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      LocaleFallbackIterator_box_destroy_registry.register(this, underlying);
    }
  }

  get() {
    return new Locale(wasm.ICU4XLocaleFallbackIterator_get(this.underlying), true, []);
  }

  step() {
    wasm.ICU4XLocaleFallbackIterator_step(this.underlying);
  }
}
