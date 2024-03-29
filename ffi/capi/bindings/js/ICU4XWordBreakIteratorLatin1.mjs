import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XSegmenterWordType_js_to_rust, ICU4XSegmenterWordType_rust_to_js } from "./ICU4XSegmenterWordType.mjs"

const ICU4XWordBreakIteratorLatin1_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XWordBreakIteratorLatin1_destroy(underlying);
});

export class ICU4XWordBreakIteratorLatin1 {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XWordBreakIteratorLatin1_box_destroy_registry.register(this, underlying);
    }
  }

  next() {
    return wasm.ICU4XWordBreakIteratorLatin1_next(this.underlying);
  }

  word_type() {
    return ICU4XSegmenterWordType_rust_to_js[wasm.ICU4XWordBreakIteratorLatin1_word_type(this.underlying)];
  }

  is_word_like() {
    return wasm.ICU4XWordBreakIteratorLatin1_is_word_like(this.underlying);
  }
}
