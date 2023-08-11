import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XCodePointRangeIteratorResult } from "./ICU4XCodePointRangeIteratorResult.js"

const ICU4XCodePointRangeIterator_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCodePointRangeIterator_destroy(underlying);
});

export class ICU4XCodePointRangeIterator {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XCodePointRangeIterator_box_destroy_registry.register(this, underlying);
    }
  }

  next() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(9, 4);
      wasm.ICU4XCodePointRangeIterator_next(diplomat_receive_buffer, this.underlying);
      const out = new ICU4XCodePointRangeIteratorResult(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 9, 4);
      return out;
    })();
  }
}
