import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"
import { ICU4XIsoTimeZoneFormat_js_to_rust, ICU4XIsoTimeZoneFormat_rust_to_js } from "./ICU4XIsoTimeZoneFormat.js"
import { ICU4XIsoTimeZoneMinuteDisplay_js_to_rust, ICU4XIsoTimeZoneMinuteDisplay_rust_to_js } from "./ICU4XIsoTimeZoneMinuteDisplay.js"
import { ICU4XIsoTimeZoneSecondDisplay_js_to_rust, ICU4XIsoTimeZoneSecondDisplay_rust_to_js } from "./ICU4XIsoTimeZoneSecondDisplay.js"

const ICU4XTimeZoneFormatter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XTimeZoneFormatter_destroy(underlying);
});

export class ICU4XTimeZoneFormatter {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XTimeZoneFormatter_box_destroy_registry.register(this, underlying);
    }
  }

  static try_new_with_localized_gmt_fallback(arg_provider, arg_locale) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XTimeZoneFormatter_try_new_with_localized_gmt_fallback(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XTimeZoneFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static try_new_with_iso_8601_fallback(arg_provider, arg_locale, arg_format, arg_minutes, arg_seconds) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XTimeZoneFormatter_try_new_with_iso_8601_fallback(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying, ICU4XIsoTimeZoneFormat_js_to_rust[arg_format], ICU4XIsoTimeZoneMinuteDisplay_js_to_rust[arg_minutes], ICU4XIsoTimeZoneSecondDisplay_js_to_rust[arg_seconds]);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XTimeZoneFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  format_custom_time_zone(arg_value) {
    return diplomatRuntime.withWriteable(wasm, (writeable) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        wasm.ICU4XTimeZoneFormatter_format_custom_time_zone(diplomat_receive_buffer, this.underlying, arg_value.underlying, writeable);
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
  }
}
