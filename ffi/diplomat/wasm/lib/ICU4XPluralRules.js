import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"
import { ICU4XPluralCategories } from "./ICU4XPluralCategories.js"
import { ICU4XPluralCategory_js_to_rust, ICU4XPluralCategory_rust_to_js } from "./ICU4XPluralCategory.js"

const ICU4XPluralRules_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XPluralRules_destroy(underlying);
});

export class ICU4XPluralRules {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XPluralRules_box_destroy_registry.register(this, underlying);
    }
  }

  static try_new_cardinal(arg_locale, arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XPluralRules_try_new_cardinal(diplomat_receive_buffer, arg_locale.underlying, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XPluralRules(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static try_new_ordinal(arg_locale, arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XPluralRules_try_new_ordinal(diplomat_receive_buffer, arg_locale.underlying, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XPluralRules(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  select(arg_op) {
    const field_i_arg_op = arg_op["i"];
    const field_v_arg_op = arg_op["v"];
    const field_w_arg_op = arg_op["w"];
    const field_f_arg_op = arg_op["f"];
    const field_t_arg_op = arg_op["t"];
    const field_c_arg_op = arg_op["c"];
    return ICU4XPluralCategory_rust_to_js[wasm.ICU4XPluralRules_select(this.underlying, field_i_arg_op, field_v_arg_op, field_w_arg_op, field_f_arg_op, field_t_arg_op, field_c_arg_op)];
  }

  categories() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(6, 1);
      wasm.ICU4XPluralRules_categories(diplomat_receive_buffer, this.underlying);
      const out = new ICU4XPluralCategories(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 6, 1);
      return out;
    })();
  }
}
