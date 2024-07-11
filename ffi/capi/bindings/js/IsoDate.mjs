import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { CalendarError_js_to_rust, CalendarError_rust_to_js } from "./CalendarError.mjs"
import { Date } from "./Date.mjs"
import { IsoWeekday_js_to_rust, IsoWeekday_rust_to_js } from "./IsoWeekday.mjs"
import { WeekOf } from "./WeekOf.mjs"
import { WeekRelativeUnit_js_to_rust, WeekRelativeUnit_rust_to_js } from "./WeekRelativeUnit.mjs"

const IsoDate_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XIsoDate_destroy(underlying);
});

export class IsoDate {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      IsoDate_box_destroy_registry.register(this, underlying);
    }
  }

  static create(arg_year, arg_month, arg_day) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XIsoDate_create(diplomat_receive_buffer, arg_year, arg_month, arg_day);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new IsoDate(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = CalendarError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_for_unix_epoch() {
    return new IsoDate(wasm.ICU4XIsoDate_create_for_unix_epoch(), true, []);
  }

  to_calendar(arg_calendar) {
    return new Date(wasm.ICU4XIsoDate_to_calendar(this.underlying, arg_calendar.underlying), true, []);
  }

  to_any() {
    return new Date(wasm.ICU4XIsoDate_to_any(this.underlying), true, []);
  }

  day_of_year() {
    return wasm.ICU4XIsoDate_day_of_year(this.underlying);
  }

  day_of_month() {
    return wasm.ICU4XIsoDate_day_of_month(this.underlying);
  }

  day_of_week() {
    return IsoWeekday_rust_to_js[wasm.ICU4XIsoDate_day_of_week(this.underlying)];
  }

  week_of_month(arg_first_weekday) {
    return wasm.ICU4XIsoDate_week_of_month(this.underlying, IsoWeekday_js_to_rust[arg_first_weekday]);
  }

  week_of_year(arg_calculator) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.ICU4XIsoDate_week_of_year(diplomat_receive_buffer, this.underlying, arg_calculator.underlying);
      const out = new WeekOf(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
  }

  month() {
    return wasm.ICU4XIsoDate_month(this.underlying);
  }

  year() {
    return wasm.ICU4XIsoDate_year(this.underlying);
  }

  is_in_leap_year() {
    return wasm.ICU4XIsoDate_is_in_leap_year(this.underlying);
  }

  months_in_year() {
    return wasm.ICU4XIsoDate_months_in_year(this.underlying);
  }

  days_in_month() {
    return wasm.ICU4XIsoDate_days_in_month(this.underlying);
  }

  days_in_year() {
    return wasm.ICU4XIsoDate_days_in_year(this.underlying);
  }
}
