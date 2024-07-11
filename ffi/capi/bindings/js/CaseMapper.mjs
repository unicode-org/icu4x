import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"
import { LeadingAdjustment_js_to_rust, LeadingAdjustment_rust_to_js } from "./LeadingAdjustment.mjs"
import { TrailingCase_js_to_rust, TrailingCase_rust_to_js } from "./TrailingCase.mjs"

const CaseMapper_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCaseMapper_destroy(underlying);
});

export class CaseMapper {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      CaseMapper_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XCaseMapper_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new CaseMapper(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  lowercase(arg_s, arg_locale) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str8(wasm, arg_s);
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XCaseMapper_lowercase(this.underlying, buf_arg_s.ptr, buf_arg_s.size, arg_locale.underlying, write);
    });
    buf_arg_s.free();
    return diplomat_out;
  }

  uppercase(arg_s, arg_locale) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str8(wasm, arg_s);
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XCaseMapper_uppercase(this.underlying, buf_arg_s.ptr, buf_arg_s.size, arg_locale.underlying, write);
    });
    buf_arg_s.free();
    return diplomat_out;
  }

  titlecase_segment_with_only_case_data_v1(arg_s, arg_locale, arg_options) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str8(wasm, arg_s);
    const field_leading_adjustment_arg_options = arg_options["leading_adjustment"];
    const field_trailing_case_arg_options = arg_options["trailing_case"];
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XCaseMapper_titlecase_segment_with_only_case_data_v1(this.underlying, buf_arg_s.ptr, buf_arg_s.size, arg_locale.underlying, LeadingAdjustment_js_to_rust[field_leading_adjustment_arg_options], TrailingCase_js_to_rust[field_trailing_case_arg_options], write);
    });
    buf_arg_s.free();
    return diplomat_out;
  }

  fold(arg_s) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str8(wasm, arg_s);
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XCaseMapper_fold(this.underlying, buf_arg_s.ptr, buf_arg_s.size, write);
    });
    buf_arg_s.free();
    return diplomat_out;
  }

  fold_turkic(arg_s) {
    const buf_arg_s = diplomatRuntime.DiplomatBuf.str8(wasm, arg_s);
    const diplomat_out = diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XCaseMapper_fold_turkic(this.underlying, buf_arg_s.ptr, buf_arg_s.size, write);
    });
    buf_arg_s.free();
    return diplomat_out;
  }

  add_case_closure_to(arg_c, arg_builder) {
    wasm.ICU4XCaseMapper_add_case_closure_to(this.underlying, diplomatRuntime.extractCodePoint(arg_c, 'arg_c'), arg_builder.underlying);
  }

  simple_lowercase(arg_ch) {
    return wasm.ICU4XCaseMapper_simple_lowercase(this.underlying, diplomatRuntime.extractCodePoint(arg_ch, 'arg_ch'));
  }

  simple_uppercase(arg_ch) {
    return wasm.ICU4XCaseMapper_simple_uppercase(this.underlying, diplomatRuntime.extractCodePoint(arg_ch, 'arg_ch'));
  }

  simple_titlecase(arg_ch) {
    return wasm.ICU4XCaseMapper_simple_titlecase(this.underlying, diplomatRuntime.extractCodePoint(arg_ch, 'arg_ch'));
  }

  simple_fold(arg_ch) {
    return wasm.ICU4XCaseMapper_simple_fold(this.underlying, diplomatRuntime.extractCodePoint(arg_ch, 'arg_ch'));
  }

  simple_fold_turkic(arg_ch) {
    return wasm.ICU4XCaseMapper_simple_fold_turkic(this.underlying, diplomatRuntime.extractCodePoint(arg_ch, 'arg_ch'));
  }
}
