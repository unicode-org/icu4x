import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"
import { LeadingAdjustment_js_to_rust, LeadingAdjustment_rust_to_js } from "./LeadingAdjustment.mjs"
import { TrailingCase_js_to_rust, TrailingCase_rust_to_js } from "./TrailingCase.mjs"

const TitlecaseMapper_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XTitlecaseMapper_destroy(underlying);
});

export class TitlecaseMapper {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      TitlecaseMapper_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XTitlecaseMapper_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new TitlecaseMapper(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  titlecase_segment_v1(arg_s, arg_locale, arg_options) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str8(wasm, arg_s);
    const field_leading_adjustment_arg_options = arg_options["leading_adjustment"];
    const field_trailing_case_arg_options = arg_options["trailing_case"];
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XTitlecaseMapper_titlecase_segment_v1(this.underlying, buf_arg_s.ptr, buf_arg_s.size, arg_locale.underlying, LeadingAdjustment_js_to_rust[field_leading_adjustment_arg_options], TrailingCase_js_to_rust[field_trailing_case_arg_options], write);
    });
    buf_arg_s.free();
    return diplomat_out;
  }
}
