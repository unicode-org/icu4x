import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DataError_js_to_rust, DataError_rust_to_js } from "./DataError.mjs"
import { DisplayNamesFallback_js_to_rust, DisplayNamesFallback_rust_to_js } from "./DisplayNamesFallback.mjs"
import { DisplayNamesStyle_js_to_rust, DisplayNamesStyle_rust_to_js } from "./DisplayNamesStyle.mjs"
import { LanguageDisplay_js_to_rust, LanguageDisplay_rust_to_js } from "./LanguageDisplay.mjs"

const LocaleDisplayNamesFormatter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XLocaleDisplayNamesFormatter_destroy(underlying);
});

export class LocaleDisplayNamesFormatter {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      LocaleDisplayNamesFormatter_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_provider, arg_locale, arg_options) {
    const field_style_arg_options = arg_options["style"];
    const field_fallback_arg_options = arg_options["fallback"];
    const field_language_display_arg_options = arg_options["language_display"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XLocaleDisplayNamesFormatter_create(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying, DisplayNamesStyle_js_to_rust[field_style_arg_options], DisplayNamesFallback_js_to_rust[field_fallback_arg_options], LanguageDisplay_js_to_rust[field_language_display_arg_options]);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new LocaleDisplayNamesFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = DataError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  of(arg_locale) {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XLocaleDisplayNamesFormatter_of(this.underlying, arg_locale.underlying, write);
    });
  }
}
