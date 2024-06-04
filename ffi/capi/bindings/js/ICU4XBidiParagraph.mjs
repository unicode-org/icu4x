import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { ICU4XBidiDirection_js_to_rust, ICU4XBidiDirection_rust_to_js } from "./ICU4XBidiDirection.mjs"

const ICU4XBidiParagraph_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XBidiParagraph_destroy(underlying);
});

export class ICU4XBidiParagraph {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XBidiParagraph_box_destroy_registry.register(this, underlying);
    }
  }

  set_paragraph_in_text(arg_n) {
    return wasm.ICU4XBidiParagraph_set_paragraph_in_text(this.underlying, arg_n);
  }

  direction() {
    return ICU4XBidiDirection_rust_to_js[wasm.ICU4XBidiParagraph_direction(this.underlying)];
  }

  size() {
    return wasm.ICU4XBidiParagraph_size(this.underlying);
  }

  range_start() {
    return wasm.ICU4XBidiParagraph_range_start(this.underlying);
  }

  range_end() {
    return wasm.ICU4XBidiParagraph_range_end(this.underlying);
  }

  reorder_line(arg_range_start, arg_range_end) {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const is_ok = wasm.ICU4XBidiParagraph_reorder_line(this.underlying, arg_range_start, arg_range_end, write) == 1;
        if (!is_ok) return;
      })();
    });
  }

  level_at(arg_pos) {
    return wasm.ICU4XBidiParagraph_level_at(this.underlying, arg_pos);
  }
}
