import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"
import { PluralCategories } from "./PluralCategories.mjs"
import { PluralCategory_js_to_rust, PluralCategory_rust_to_js } from "./PluralCategory.mjs"

const PluralRules_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XPluralRules_destroy(underlying);
});

export class PluralRules {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      PluralRules_box_destroy_registry.register(this, underlying);
    }
  }

  static create_cardinal(arg_provider, arg_locale) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XPluralRules_create_cardinal(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new PluralRules(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_ordinal(arg_provider, arg_locale) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XPluralRules_create_ordinal(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new PluralRules(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  category_for(arg_op) {
    return PluralCategory_rust_to_js[wasm.ICU4XPluralRules_category_for(this.underlying, arg_op.underlying)];
  }

  categories() {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(6, 1);
      wasm.ICU4XPluralRules_categories(diplomat_receive_buffer, this.underlying);
      const out = new PluralCategories(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 6, 1);
      return out;
    })();
  }
}
