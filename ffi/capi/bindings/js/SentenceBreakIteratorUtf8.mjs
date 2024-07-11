import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const SentenceBreakIteratorUtf8_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XSentenceBreakIteratorUtf8_destroy(underlying);
});

export class SentenceBreakIteratorUtf8 {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      SentenceBreakIteratorUtf8_box_destroy_registry.register(this, underlying);
    }
  }

  next() {
    return wasm.ICU4XSentenceBreakIteratorUtf8_next(this.underlying);
  }
}
