import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { FixedDecimalLimitError_js_to_rust, FixedDecimalLimitError_rust_to_js } from "./FixedDecimalLimitError.mjs"
import { FixedDecimalParseError_js_to_rust, FixedDecimalParseError_rust_to_js } from "./FixedDecimalParseError.mjs"
import { FixedDecimalRoundingIncrement_js_to_rust, FixedDecimalRoundingIncrement_rust_to_js } from "./FixedDecimalRoundingIncrement.mjs"
import { FixedDecimalRoundingMode_js_to_rust, FixedDecimalRoundingMode_rust_to_js } from "./FixedDecimalRoundingMode.mjs"
import { FixedDecimalSign_js_to_rust, FixedDecimalSign_rust_to_js } from "./FixedDecimalSign.mjs"
import { FixedDecimalSignDisplay_js_to_rust, FixedDecimalSignDisplay_rust_to_js } from "./FixedDecimalSignDisplay.mjs"

const FixedDecimal_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XFixedDecimal_destroy(underlying);
});

export class FixedDecimal {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      FixedDecimal_box_destroy_registry.register(this, underlying);
    }
  }

  static create_from_i32(arg_v) {
    return new FixedDecimal(wasm.ICU4XFixedDecimal_create_from_i32(arg_v), true, []);
  }

  static create_from_u32(arg_v) {
    return new FixedDecimal(wasm.ICU4XFixedDecimal_create_from_u32(arg_v), true, []);
  }

  static create_from_i64(arg_v) {
    return new FixedDecimal(wasm.ICU4XFixedDecimal_create_from_i64(arg_v), true, []);
  }

  static create_from_u64(arg_v) {
    return new FixedDecimal(wasm.ICU4XFixedDecimal_create_from_u64(arg_v), true, []);
  }

  static create_from_f64_with_integer_precision(arg_f) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XFixedDecimal_create_from_f64_with_integer_precision(diplomat_receive_buffer, arg_f);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new FixedDecimal(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = FixedDecimalLimitError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_from_f64_with_lower_magnitude(arg_f, arg_magnitude) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XFixedDecimal_create_from_f64_with_lower_magnitude(diplomat_receive_buffer, arg_f, arg_magnitude);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new FixedDecimal(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = FixedDecimalLimitError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_from_f64_with_significant_digits(arg_f, arg_digits) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XFixedDecimal_create_from_f64_with_significant_digits(diplomat_receive_buffer, arg_f, arg_digits);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new FixedDecimal(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = FixedDecimalLimitError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_from_f64_with_floating_precision(arg_f) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XFixedDecimal_create_from_f64_with_floating_precision(diplomat_receive_buffer, arg_f);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new FixedDecimal(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = FixedDecimalLimitError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_from_string(arg_v) {
    const buf_arg_v = diplomatRuntime.DiplomatBuf.str8(wasm, arg_v);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XFixedDecimal_create_from_string(diplomat_receive_buffer, buf_arg_v.ptr, buf_arg_v.size);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new FixedDecimal(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = FixedDecimalParseError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_v.free();
    return diplomat_out;
  }

  digit_at(arg_magnitude) {
    return wasm.ICU4XFixedDecimal_digit_at(this.underlying, arg_magnitude);
  }

  magnitude_start() {
    return wasm.ICU4XFixedDecimal_magnitude_start(this.underlying);
  }

  magnitude_end() {
    return wasm.ICU4XFixedDecimal_magnitude_end(this.underlying);
  }

  nonzero_magnitude_start() {
    return wasm.ICU4XFixedDecimal_nonzero_magnitude_start(this.underlying);
  }

  nonzero_magnitude_end() {
    return wasm.ICU4XFixedDecimal_nonzero_magnitude_end(this.underlying);
  }

  is_zero() {
    return wasm.ICU4XFixedDecimal_is_zero(this.underlying);
  }

  multiply_pow10(arg_power) {
    wasm.ICU4XFixedDecimal_multiply_pow10(this.underlying, arg_power);
  }

  sign() {
    return FixedDecimalSign_rust_to_js[wasm.ICU4XFixedDecimal_sign(this.underlying)];
  }

  set_sign(arg_sign) {
    wasm.ICU4XFixedDecimal_set_sign(this.underlying, FixedDecimalSign_js_to_rust[arg_sign]);
  }

  apply_sign_display(arg_sign_display) {
    wasm.ICU4XFixedDecimal_apply_sign_display(this.underlying, FixedDecimalSignDisplay_js_to_rust[arg_sign_display]);
  }

  trim_start() {
    wasm.ICU4XFixedDecimal_trim_start(this.underlying);
  }

  trim_end() {
    wasm.ICU4XFixedDecimal_trim_end(this.underlying);
  }

  pad_start(arg_position) {
    wasm.ICU4XFixedDecimal_pad_start(this.underlying, arg_position);
  }

  pad_end(arg_position) {
    wasm.ICU4XFixedDecimal_pad_end(this.underlying, arg_position);
  }

  set_max_position(arg_position) {
    wasm.ICU4XFixedDecimal_set_max_position(this.underlying, arg_position);
  }

  round(arg_position) {
    wasm.ICU4XFixedDecimal_round(this.underlying, arg_position);
  }

  ceil(arg_position) {
    wasm.ICU4XFixedDecimal_ceil(this.underlying, arg_position);
  }

  expand(arg_position) {
    wasm.ICU4XFixedDecimal_expand(this.underlying, arg_position);
  }

  floor(arg_position) {
    wasm.ICU4XFixedDecimal_floor(this.underlying, arg_position);
  }

  trunc(arg_position) {
    wasm.ICU4XFixedDecimal_trunc(this.underlying, arg_position);
  }

  round_with_mode(arg_position, arg_mode) {
    wasm.ICU4XFixedDecimal_round_with_mode(this.underlying, arg_position, FixedDecimalRoundingMode_js_to_rust[arg_mode]);
  }

  round_with_mode_and_increment(arg_position, arg_mode, arg_increment) {
    wasm.ICU4XFixedDecimal_round_with_mode_and_increment(this.underlying, arg_position, FixedDecimalRoundingMode_js_to_rust[arg_mode], FixedDecimalRoundingIncrement_js_to_rust[arg_increment]);
  }

  concatenate_end(arg_other) {
    return (() => {
      const is_ok = wasm.ICU4XFixedDecimal_concatenate_end(this.underlying, arg_other.underlying) == 1;
      if (!is_ok) {
        throw new diplomatRuntime.FFIError(undefined);
      }
    })();
  }

  to_string() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XFixedDecimal_to_string(this.underlying, write);
    });
  }
}
