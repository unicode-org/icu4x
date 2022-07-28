import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XCanonicalizationResult_js_to_rust, ICU4XCanonicalizationResult_rust_to_js } from "./ICU4XCanonicalizationResult.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"

const ICU4XLocaleCanonicalizer_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLocaleCanonicalizer_destroy(underlying);
});

export class ICU4XLocaleCanonicalizer {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XLocaleCanonicalizer_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLocaleCanonicalizer_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XLocaleCanonicalizer(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  canonicalize(arg_locale) {
    return ICU4XCanonicalizationResult_rust_to_js[wasm.ICU4XLocaleCanonicalizer_canonicalize(this.underlying, arg_locale.underlying)];
  }

  maximize(arg_locale) {
    return ICU4XCanonicalizationResult_rust_to_js[wasm.ICU4XLocaleCanonicalizer_maximize(this.underlying, arg_locale.underlying)];
  }

  minimize(arg_locale) {
    return ICU4XCanonicalizationResult_rust_to_js[wasm.ICU4XLocaleCanonicalizer_minimize(this.underlying, arg_locale.underlying)];
  }
}
