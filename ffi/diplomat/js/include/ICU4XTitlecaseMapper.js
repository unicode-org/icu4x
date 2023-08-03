import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"
import { ICU4XHeadAdjustment_js_to_rust, ICU4XHeadAdjustment_rust_to_js } from "./ICU4XHeadAdjustment.js"
import { ICU4XTailCasing_js_to_rust, ICU4XTailCasing_rust_to_js } from "./ICU4XTailCasing.js"

const ICU4XTitlecaseMapper_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XTitlecaseMapper_destroy(underlying);
});

export class ICU4XTitlecaseMapper {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XTitlecaseMapper_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XTitlecaseMapper_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XTitlecaseMapper(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_legacy(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XTitlecaseMapper_create_legacy(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XTitlecaseMapper(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  titlecase_segment(arg_s, arg_locale, arg_options) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str(wasm, arg_s);
    const field_head_adjustment_arg_options = arg_options["head_adjustment"];
    const field_tail_casing_arg_options = arg_options["tail_casing"];
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        wasm.ICU4XTitlecaseMapper_titlecase_segment(diplomat_receive_buffer, this.underlying, buf_arg_s.ptr, buf_arg_s.size, arg_locale.underlying, ICU4XHeadAdjustment_js_to_rust[field_head_adjustment_arg_options], ICU4XTailCasing_js_to_rust[field_tail_casing_arg_options], writeable);
        const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
        if (is_ok) {
          const ok_value = {};
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          return ok_value;
        } else {
          const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    buf_arg_s.free();
    return diplomat_out;
  }
}
