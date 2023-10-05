import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"
import { ICU4XPluralCategory_js_to_rust, ICU4XPluralCategory_rust_to_js } from "./ICU4XPluralCategory.js"

const ICU4XPluralRanges_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XPluralRanges_destroy(underlying);
});

export class ICU4XPluralRanges {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XPluralRanges_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider, arg_locale) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XPluralRanges_create(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XPluralRanges(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  category_for_range(arg_start, arg_end) {
    return ICU4XPluralCategory_rust_to_js[wasm.ICU4XPluralRanges_category_for_range(this.underlying, ICU4XPluralCategory_js_to_rust[arg_start], ICU4XPluralCategory_js_to_rust[arg_end])];
  }
}
