import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"

const ICU4XCanonicalCombiningClassMap_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XCanonicalCombiningClassMap_destroy(underlying);
});

export class ICU4XCanonicalCombiningClassMap {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XCanonicalCombiningClassMap_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XCanonicalCombiningClassMap_create(diplomat_receive_buffer, arg_provider.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XCanonicalCombiningClassMap(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  get(arg_ch) {
    return wasm.ICU4XCanonicalCombiningClassMap_get(this.underlying, diplomatRuntime.extractCodePoint(arg_ch, 'arg_ch'));
  }

  get32(arg_ch) {
    return wasm.ICU4XCanonicalCombiningClassMap_get32(this.underlying, arg_ch);
  }
}
