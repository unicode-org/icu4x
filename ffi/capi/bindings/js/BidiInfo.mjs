import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { BidiParagraph } from "./BidiParagraph.mjs"

const BidiInfo_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XBidiInfo_destroy(underlying);
});

export class BidiInfo {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      BidiInfo_box_destroy_registry.register(this, underlying);
    }
  }

  paragraph_count() {
    return wasm.ICU4XBidiInfo_paragraph_count(this.underlying);
  }

  paragraph_at(arg_n) {
    return (() => {
      const option_ptr = wasm.ICU4XBidiInfo_paragraph_at(this.underlying, arg_n);
      return (option_ptr == 0) ? undefined : new BidiParagraph(option_ptr, true, [this]);
    })();
  }

  size() {
    return wasm.ICU4XBidiInfo_size(this.underlying);
  }

  level_at(arg_pos) {
    return wasm.ICU4XBidiInfo_level_at(this.underlying, arg_pos);
  }
}
