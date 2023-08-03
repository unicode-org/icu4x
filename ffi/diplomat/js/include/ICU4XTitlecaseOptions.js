import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XHeadAdjustment_js_to_rust, ICU4XHeadAdjustment_rust_to_js } from "./ICU4XHeadAdjustment.js"
import { ICU4XTailCasing_js_to_rust, ICU4XTailCasing_rust_to_js } from "./ICU4XTailCasing.js"

export class ICU4XTitlecaseOptions {
  constructor(underlying) {
    this.head_adjustment = ICU4XHeadAdjustment_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.tail_casing = ICU4XTailCasing_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
  }

  static default_options() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.ICU4XTitlecaseOptions_default_options(diplomat_receive_buffer);
      const out = new ICU4XTitlecaseOptions(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
  }
}
