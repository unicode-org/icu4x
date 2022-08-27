import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"
import { ICU4XDateTime } from "./ICU4XDateTime.js"
import { ICU4XError_js_to_rust, ICU4XError_rust_to_js } from "./ICU4XError.js"
import { ICU4XIsoDate } from "./ICU4XIsoDate.js"
import { ICU4XTime } from "./ICU4XTime.js"

const ICU4XIsoDateTime_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XIsoDateTime_destroy(underlying);
});

export class ICU4XIsoDateTime {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      ICU4XIsoDateTime_box_destroy_registry.register(this, underlying);
    }
  }

  static try_new(arg_year, arg_month, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XIsoDateTime_try_new(diplomat_receive_buffer, arg_year, arg_month, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XIsoDateTime(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static new_from_date_and_time(arg_date, arg_time) {
    return new ICU4XIsoDateTime(wasm.ICU4XIsoDateTime_new_from_date_and_time(arg_date.underlying, arg_time.underlying), true, []);
  }

  static from_minutes_since_local_unix_epoch(arg_minutes) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XIsoDateTime_from_minutes_since_local_unix_epoch(diplomat_receive_buffer, arg_minutes);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new ICU4XIsoDateTime(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = ICU4XError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  date() {
    return new ICU4XIsoDate(wasm.ICU4XIsoDateTime_date(this.underlying), true, []);
  }

  time() {
    return new ICU4XTime(wasm.ICU4XIsoDateTime_time(this.underlying), true, []);
  }

  to_any() {
    return new ICU4XDateTime(wasm.ICU4XIsoDateTime_to_any(this.underlying), true, []);
  }

  minutes_since_local_unix_epoch() {
    return wasm.ICU4XIsoDateTime_minutes_since_local_unix_epoch(this.underlying);
  }
}
