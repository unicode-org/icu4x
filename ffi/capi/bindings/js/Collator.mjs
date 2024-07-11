import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { CollatorAlternateHandling_js_to_rust, CollatorAlternateHandling_rust_to_js } from "./CollatorAlternateHandling.mjs"
import { CollatorBackwardSecondLevel_js_to_rust, CollatorBackwardSecondLevel_rust_to_js } from "./CollatorBackwardSecondLevel.mjs"
import { CollatorCaseFirst_js_to_rust, CollatorCaseFirst_rust_to_js } from "./CollatorCaseFirst.mjs"
import { CollatorCaseLevel_js_to_rust, CollatorCaseLevel_rust_to_js } from "./CollatorCaseLevel.mjs"
import { CollatorMaxVariable_js_to_rust, CollatorMaxVariable_rust_to_js } from "./CollatorMaxVariable.mjs"
import { CollatorNumeric_js_to_rust, CollatorNumeric_rust_to_js } from "./CollatorNumeric.mjs"
import { CollatorResolvedOptionsV1 } from "./CollatorResolvedOptionsV1.mjs"
import { CollatorStrength_js_to_rust, CollatorStrength_rust_to_js } from "./CollatorStrength.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"
import { Ordering_js_to_rust, Ordering_rust_to_js } from "./Ordering.mjs"

const Collator_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCollator_destroy(underlying);
});

export class Collator {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      Collator_box_destroy_registry.register(this, underlying);
    }
  }

  static create_v1(arg_provider, arg_locale, arg_options) {
    const field_strength_arg_options = arg_options["strength"];
    const field_alternate_handling_arg_options = arg_options["alternate_handling"];
    const field_case_first_arg_options = arg_options["case_first"];
    const field_max_variable_arg_options = arg_options["max_variable"];
    const field_case_level_arg_options = arg_options["case_level"];
    const field_numeric_arg_options = arg_options["numeric"];
    const field_backward_second_level_arg_options = arg_options["backward_second_level"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XCollator_create_v1(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying, CollatorStrength_js_to_rust[field_strength_arg_options], CollatorAlternateHandling_js_to_rust[field_alternate_handling_arg_options], CollatorCaseFirst_js_to_rust[field_case_first_arg_options], CollatorMaxVariable_js_to_rust[field_max_variable_arg_options], CollatorCaseLevel_js_to_rust[field_case_level_arg_options], CollatorNumeric_js_to_rust[field_numeric_arg_options], CollatorBackwardSecondLevel_js_to_rust[field_backward_second_level_arg_options]);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new Collator(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  compare(arg_left, arg_right) {
    const buf_arg_left = diplomatRuntime.DiplomatBuf.str8(wasm, arg_left);
    const buf_arg_right = diplomatRuntime.DiplomatBuf.str8(wasm, arg_right);
    const diplomat_out = Ordering_rust_to_js[wasm.ICU4XCollator_compare(this.underlying, buf_arg_left.ptr, buf_arg_left.size, buf_arg_right.ptr, buf_arg_right.size)];
    buf_arg_left.free();
    buf_arg_right.free();
    return diplomat_out;
  }

  compare_valid_utf8(arg_left, arg_right) {
    const buf_arg_left = diplomatRuntime.DiplomatBuf.str8(wasm, arg_left);
    const buf_arg_right = diplomatRuntime.DiplomatBuf.str8(wasm, arg_right);
    const diplomat_out = Ordering_rust_to_js[wasm.ICU4XCollator_compare_valid_utf8(this.underlying, buf_arg_left.ptr, buf_arg_left.size, buf_arg_right.ptr, buf_arg_right.size)];
    buf_arg_left.free();
    buf_arg_right.free();
    return diplomat_out;
  }

  compare_utf16(arg_left, arg_right) {
    const buf_arg_left = diplomatRuntime.DiplomatBuf.str16(wasm, arg_left);
    const buf_arg_right = diplomatRuntime.DiplomatBuf.str16(wasm, arg_right);
    const diplomat_out = Ordering_rust_to_js[wasm.ICU4XCollator_compare_utf16(this.underlying, buf_arg_left.ptr, buf_arg_left.size, buf_arg_right.ptr, buf_arg_right.size)];
    buf_arg_left.free();
    buf_arg_right.free();
    return diplomat_out;
  }

  resolved_options() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(28, 4);
      wasm.ICU4XCollator_resolved_options(diplomat_receive_buffer, this.underlying);
      const out = new CollatorResolvedOptionsV1(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 28, 4);
      return out;
    })();
  }
}
