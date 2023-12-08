import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

const ICU4XSentenceBreakIteratorLatin1_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XSentenceBreakIteratorLatin1_destroy(underlying);
});

export class ICU4XSentenceBreakIteratorLatin1 {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XSentenceBreakIteratorLatin1_box_destroy_registry.register(this, underlying);
    }
  }

  next() {
    return wasm.ICU4XSentenceBreakIteratorLatin1_next(this.underlying);
  }
}
