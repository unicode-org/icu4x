import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { DateLength_js_to_rust, DateLength_rust_to_js } from "./DateLength.mjs"
import { Error_js_to_rust, Error_rust_to_js } from "./Error.mjs"
import { IsoTimeZoneFormat_js_to_rust, IsoTimeZoneFormat_rust_to_js } from "./IsoTimeZoneFormat.mjs"
import { IsoTimeZoneMinuteDisplay_js_to_rust, IsoTimeZoneMinuteDisplay_rust_to_js } from "./IsoTimeZoneMinuteDisplay.mjs"
import { IsoTimeZoneSecondDisplay_js_to_rust, IsoTimeZoneSecondDisplay_rust_to_js } from "./IsoTimeZoneSecondDisplay.mjs"
import { TimeLength_js_to_rust, TimeLength_rust_to_js } from "./TimeLength.mjs"

const ZonedDateTimeFormatter_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XZonedDateTimeFormatter_destroy(underlying);
});

export class ZonedDateTimeFormatter {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ZonedDateTimeFormatter_box_destroy_registry.register(this, underlying);
    }
  }

  static create_with_lengths(arg_provider, arg_locale, arg_date_length, arg_time_length) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XZonedDateTimeFormatter_create_with_lengths(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying, DateLength_js_to_rust[arg_date_length], TimeLength_js_to_rust[arg_time_length]);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ZonedDateTimeFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = Error_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_with_lengths_and_iso_8601_time_zone_fallback(arg_provider, arg_locale, arg_date_length, arg_time_length, arg_zone_options) {
    const field_format_arg_zone_options = arg_zone_options["format"];
    const field_minutes_arg_zone_options = arg_zone_options["minutes"];
    const field_seconds_arg_zone_options = arg_zone_options["seconds"];
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback(diplomat_receive_buffer, arg_provider.underlying, arg_locale.underlying, DateLength_js_to_rust[arg_date_length], TimeLength_js_to_rust[arg_time_length], IsoTimeZoneFormat_js_to_rust[field_format_arg_zone_options], IsoTimeZoneMinuteDisplay_js_to_rust[field_minutes_arg_zone_options], IsoTimeZoneSecondDisplay_js_to_rust[field_seconds_arg_zone_options]);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ZonedDateTimeFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = Error_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  format_datetime_with_custom_time_zone(arg_datetime, arg_time_zone) {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        wasm.ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone(diplomat_receive_buffer, this.underlying, arg_datetime.underlying, arg_time_zone.underlying, write);
        const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
        if (is_ok) {
          const ok_value = {};
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          return ok_value;
        } else {
          const throw_value = Error_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
  }

  format_iso_datetime_with_custom_time_zone(arg_datetime, arg_time_zone) {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return (() => {
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        wasm.ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone(diplomat_receive_buffer, this.underlying, arg_datetime.underlying, arg_time_zone.underlying, write);
        const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
        if (is_ok) {
          const ok_value = {};
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          return ok_value;
        } else {
          const throw_value = Error_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
          wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
          throw new diplomatRuntime.FFIError(throw_value);
        }
      })();
    });
  }
}
