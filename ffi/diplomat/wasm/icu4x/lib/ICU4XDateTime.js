import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XDate } from "./ICU4XDate.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"
import { ICU4XIsoDateTime } from "./ICU4XIsoDateTime.js"
import { ICU4XTime } from "./ICU4XTime.js"

const ICU4XDateTime_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XDateTime_destroy(underlying);
});

export class ICU4XDateTime {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XDateTime_box_destroy_registry.register(this, underlying);
    }
  }

  static try_new_from_iso_in_calendar(arg_year, arg_month, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond, arg_calendar) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDateTime_try_new_from_iso_in_calendar(diplomat_receive_buffer, arg_year, arg_month, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond, arg_calendar.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XDateTime(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static try_new_from_codes_in_calendar(arg_era_code, arg_year, arg_month_code, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond, arg_calendar) {
    const buf_arg_era_code = diplomatRuntime.DiplomatBuf.str(wasm, arg_era_code);
    const buf_arg_month_code = diplomatRuntime.DiplomatBuf.str(wasm, arg_month_code);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDateTime_try_new_from_codes_in_calendar(diplomat_receive_buffer, buf_arg_era_code.ptr, buf_arg_era_code.size, arg_year, buf_arg_month_code.ptr, buf_arg_month_code.size, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond, arg_calendar.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XDateTime(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_era_code.free();
    buf_arg_month_code.free();
    return diplomat_out;
  }

  static new_from_date_and_time(arg_date, arg_time) {
    return new ICU4XDateTime(wasm.ICU4XDateTime_new_from_date_and_time(arg_date.underlying, arg_time.underlying), true, []);
  }

  date() {
    return new ICU4XDate(wasm.ICU4XDateTime_date(this.underlying), true, []);
  }

  time() {
    return new ICU4XTime(wasm.ICU4XDateTime_time(this.underlying), true, []);
  }

  to_iso() {
    return new ICU4XIsoDateTime(wasm.ICU4XDateTime_to_iso(this.underlying), true, []);
  }

  to_calendar(arg_calendar) {
    return new ICU4XDateTime(wasm.ICU4XDateTime_to_calendar(this.underlying, arg_calendar.underlying), true, []);
  }
}
