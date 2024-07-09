import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"

const ICU4XDataStruct_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XDataStruct_destroy(underlying);
});

export class ICU4XDataStruct {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XDataStruct_box_destroy_registry.register(this, underlying);
    }
  }

  static create_decimal_symbols_v1(arg_plus_sign_prefix, arg_plus_sign_suffix, arg_minus_sign_prefix, arg_minus_sign_suffix, arg_decimal_separator, arg_grouping_separator, arg_primary_group_size, arg_secondary_group_size, arg_min_group_size, arg_digits) {
    const buf_arg_plus_sign_prefix = diplomatRuntime.DiplomatBuf.str8(wasm, arg_plus_sign_prefix);
    const buf_arg_plus_sign_suffix = diplomatRuntime.DiplomatBuf.str8(wasm, arg_plus_sign_suffix);
    const buf_arg_minus_sign_prefix = diplomatRuntime.DiplomatBuf.str8(wasm, arg_minus_sign_prefix);
    const buf_arg_minus_sign_suffix = diplomatRuntime.DiplomatBuf.str8(wasm, arg_minus_sign_suffix);
    const buf_arg_decimal_separator = diplomatRuntime.DiplomatBuf.str8(wasm, arg_decimal_separator);
    const buf_arg_grouping_separator = diplomatRuntime.DiplomatBuf.str8(wasm, arg_grouping_separator);
    const buf_arg_digits = diplomatRuntime.DiplomatBuf.slice(wasm, arg_digits, "char");
    const diplomat_out = (() => {
      const option_ptr = wasm.ICU4XDataStruct_create_decimal_symbols_v1(buf_arg_plus_sign_prefix.ptr, buf_arg_plus_sign_prefix.size, buf_arg_plus_sign_suffix.ptr, buf_arg_plus_sign_suffix.size, buf_arg_minus_sign_prefix.ptr, buf_arg_minus_sign_prefix.size, buf_arg_minus_sign_suffix.ptr, buf_arg_minus_sign_suffix.size, buf_arg_decimal_separator.ptr, buf_arg_decimal_separator.size, buf_arg_grouping_separator.ptr, buf_arg_grouping_separator.size, arg_primary_group_size, arg_secondary_group_size, arg_min_group_size, buf_arg_digits.ptr, buf_arg_digits.size);
      return (option_ptr == 0) ? undefined : new ICU4XDataStruct(option_ptr, true, []);
    })();
    buf_arg_plus_sign_prefix.free();
    buf_arg_plus_sign_suffix.free();
    buf_arg_minus_sign_prefix.free();
    buf_arg_minus_sign_suffix.free();
    buf_arg_decimal_separator.free();
    buf_arg_grouping_separator.free();
    buf_arg_digits.free();
    return diplomat_out;
  }
}
