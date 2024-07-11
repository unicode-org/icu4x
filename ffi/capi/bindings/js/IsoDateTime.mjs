import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { CalendarError_js_to_rust, CalendarError_rust_to_js } from "./CalendarError.mjs"
import { DateTime } from "./DateTime.mjs"
import { IsoDate } from "./IsoDate.mjs"
import { IsoWeekday_js_to_rust, IsoWeekday_rust_to_js } from "./IsoWeekday.mjs"
import { Time } from "./Time.mjs"
import { WeekOf } from "./WeekOf.mjs"
import { WeekRelativeUnit_js_to_rust, WeekRelativeUnit_rust_to_js } from "./WeekRelativeUnit.mjs"

const IsoDateTime_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XIsoDateTime_destroy(underlying);
});

export class IsoDateTime {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      IsoDateTime_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_year, arg_month, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XIsoDateTime_create(diplomat_receive_buffer, arg_year, arg_month, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new IsoDateTime(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = CalendarError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static crate_from_date_and_time(arg_date, arg_time) {
    return new IsoDateTime(wasm.ICU4XIsoDateTime_crate_from_date_and_time(arg_date.underlying, arg_time.underlying), true, []);
  }

  static local_unix_epoch() {
    return new IsoDateTime(wasm.ICU4XIsoDateTime_local_unix_epoch(), true, []);
  }

  static create_from_minutes_since_local_unix_epoch(arg_minutes) {
    return new IsoDateTime(wasm.ICU4XIsoDateTime_create_from_minutes_since_local_unix_epoch(arg_minutes), true, []);
  }

  date() {
    return new IsoDate(wasm.ICU4XIsoDateTime_date(this.underlying), true, []);
  }

  time() {
    return new Time(wasm.ICU4XIsoDateTime_time(this.underlying), true, []);
  }

  to_any() {
    return new DateTime(wasm.ICU4XIsoDateTime_to_any(this.underlying), true, []);
  }

  minutes_since_local_unix_epoch() {
    return wasm.ICU4XIsoDateTime_minutes_since_local_unix_epoch(this.underlying);
  }

  to_calendar(arg_calendar) {
    return new DateTime(wasm.ICU4XIsoDateTime_to_calendar(this.underlying, arg_calendar.underlying), true, []);
  }

  hour() {
    return wasm.ICU4XIsoDateTime_hour(this.underlying);
  }

  minute() {
    return wasm.ICU4XIsoDateTime_minute(this.underlying);
  }

  second() {
    return wasm.ICU4XIsoDateTime_second(this.underlying);
  }

  nanosecond() {
    return wasm.ICU4XIsoDateTime_nanosecond(this.underlying);
  }

  day_of_year() {
    return wasm.ICU4XIsoDateTime_day_of_year(this.underlying);
  }

  day_of_month() {
    return wasm.ICU4XIsoDateTime_day_of_month(this.underlying);
  }

  day_of_week() {
    return IsoWeekday_rust_to_js[wasm.ICU4XIsoDateTime_day_of_week(this.underlying)];
  }

  week_of_month(arg_first_weekday) {
    return wasm.ICU4XIsoDateTime_week_of_month(this.underlying, IsoWeekday_js_to_rust[arg_first_weekday]);
  }

  week_of_year(arg_calculator) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.ICU4XIsoDateTime_week_of_year(diplomat_receive_buffer, this.underlying, arg_calculator.underlying);
      const out = new WeekOf(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
  }

  month() {
    return wasm.ICU4XIsoDateTime_month(this.underlying);
  }

  year() {
    return wasm.ICU4XIsoDateTime_year(this.underlying);
  }

  is_in_leap_year() {
    return wasm.ICU4XIsoDateTime_is_in_leap_year(this.underlying);
  }

  months_in_year() {
    return wasm.ICU4XIsoDateTime_months_in_year(this.underlying);
  }

  days_in_month() {
    return wasm.ICU4XIsoDateTime_days_in_month(this.underlying);
  }

  days_in_year() {
    return wasm.ICU4XIsoDateTime_days_in_year(this.underlying);
  }
}
