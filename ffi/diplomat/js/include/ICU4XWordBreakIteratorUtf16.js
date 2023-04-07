import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XSegmenterRuleStatusType_js_to_rust, ICU4XSegmenterRuleStatusType_rust_to_js } from "./ICU4XSegmenterRuleStatusType.js"

const ICU4XWordBreakIteratorUtf16_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XWordBreakIteratorUtf16_destroy(underlying);
});

export class ICU4XWordBreakIteratorUtf16 {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XWordBreakIteratorUtf16_box_destroy_registry.register(this, underlying);
    }
  }

  next() {
    return wasm.ICU4XWordBreakIteratorUtf16_next(this.underlying);
  }

  rule_status() {
    return ICU4XSegmenterRuleStatusType_rust_to_js[wasm.ICU4XWordBreakIteratorUtf16_rule_status(this.underlying)];
  }

  is_word_like() {
    return wasm.ICU4XWordBreakIteratorUtf16_is_word_like(this.underlying);
  }
}
