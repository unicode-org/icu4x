import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { LeadingAdjustment_js_to_rust, LeadingAdjustment_rust_to_js } from "./LeadingAdjustment.mjs"
import { TrailingCase_js_to_rust, TrailingCase_rust_to_js } from "./TrailingCase.mjs"

export class TitlecaseOptionsV1 {
  constructor(underlying) {
    this.leading_adjustment = LeadingAdjustment_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying)];
    this.trailing_case = TrailingCase_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, underlying + 4)];
  }

  static default_options() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.ICU4XTitlecaseOptionsV1_default_options(diplomat_receive_buffer);
      const out = new TitlecaseOptionsV1(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
  }
}
