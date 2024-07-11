import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { SegmenterWordType_js_to_rust, SegmenterWordType_rust_to_js } from "./SegmenterWordType.mjs"

const WordBreakIteratorUtf16_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XWordBreakIteratorUtf16_destroy(underlying);
});

export class WordBreakIteratorUtf16 {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      WordBreakIteratorUtf16_box_destroy_registry.register(this, underlying);
    }
  }

  next() {
    return wasm.ICU4XWordBreakIteratorUtf16_next(this.underlying);
  }

  word_type() {
    return SegmenterWordType_rust_to_js[wasm.ICU4XWordBreakIteratorUtf16_word_type(this.underlying)];
  }

  is_word_like() {
    return wasm.ICU4XWordBreakIteratorUtf16_is_word_like(this.underlying);
  }
}
