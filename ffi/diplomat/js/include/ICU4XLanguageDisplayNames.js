import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"
import { ICU4XFallback_js_to_rust, ICU4XFallback_rust_to_js } from "./ICU4XFallback.js"
import { ICU4XLanguageDisplay_js_to_rust, ICU4XLanguageDisplay_rust_to_js } from "./ICU4XLanguageDisplay.js"
import { ICU4XStyle_js_to_rust, ICU4XStyle_rust_to_js } from "./ICU4XStyle.js"

const ICU4XLanguageDisplayNames_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLanguageDisplayNames_destroy(underlying);
});

export class ICU4XLanguageDisplayNames {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XLanguageDisplayNames_box_destroy_registry.register(this, underlying);
    }
  }

  static try_new_unstable(arg_provider, arg_locale, arg_options) {
    const field_style_arg_options = arg_options["style"];
    const field_fallback_arg_options = arg_options["fallback"];
    const field_language_display_arg_options = arg_options["language_display"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLanguageDisplayNames_try_new_unstable(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying, ICU4XStyle_js_to_rust[field_style_arg_options], ICU4XFallback_js_to_rust[field_fallback_arg_options], ICU4XLanguageDisplay_js_to_rust[field_language_display_arg_options]);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XLanguageDisplayNames(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
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
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        wasm.ICU4XLanguageDisplayNames_of(diplomat_receive_buffer, this.underlying, buf_arg_code.ptr, buf_arg_code.size, writeable);
        const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
        if (is_ok) {
          const ok_value = {};
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          return ok_value;
        } else {
          const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
    buf_arg_code.free();
    return diplomat_out;
  }
}
