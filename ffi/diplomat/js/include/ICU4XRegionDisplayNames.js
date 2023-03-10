import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"

const ICU4XRegionDisplayNames_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XRegionDisplayNames_destroy(underlying);
});

export class ICU4XRegionDisplayNames {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XRegionDisplayNames_box_destroy_registry.register(this, underlying);
    }
  }

  static try_new_unstable(arg_provider, arg_locale) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XRegionDisplayNames_try_new_unstable(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XRegionDisplayNames(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  of(arg_code) {
    const buf_arg_code = diplomatRuntime.DiplomatBuf.str(wasm, arg_code);
    const diplomat_out = diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const is_ok = wasm.ICU4XRegionDisplayNames_of(this.underlying, buf_arg_code.ptr, buf_arg_code.size, writeable) == 1;
        if (!is_ok) {
          throw new diplomatRuntime.FFIError(undefined);
        }
      })();
    });
    buf_arg_code.free();
    return diplomat_out;
  }
}
