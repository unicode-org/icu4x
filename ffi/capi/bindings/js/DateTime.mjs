import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.mjs"
import { Calendar } from "./Calendar.mjs"
import { CalendarError_js_to_rust, CalendarError_rust_to_js } from "./CalendarError.mjs"
import { Date } from "./Date.mjs"
import { IsoDateTime } from "./IsoDateTime.mjs"
import { IsoWeekday_js_to_rust, IsoWeekday_rust_to_js } from "./IsoWeekday.mjs"
import { Time } from "./Time.mjs"
import { WeekOf } from "./WeekOf.mjs"
import { WeekRelativeUnit_js_to_rust, WeekRelativeUnit_rust_to_js } from "./WeekRelativeUnit.mjs"

const DateTime_box_destroy_registry = new FinalizationRegistry(underlying => {
  wasm.ICU4XDateTime_destroy(underlying);
});

export class DateTime {
  #lifetimeEdges = [];
  constructor(underlying, owned, edges) {
    this.underlying = underlying;
    this.#lifetimeEdges.push(...edges);
    if (owned) {
      DateTime_box_destroy_registry.register(this, underlying);
    }
  }

  static create_from_iso_in_calendar(arg_year, arg_month, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond, arg_calendar) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDateTime_create_from_iso_in_calendar(diplomat_receive_buffer, arg_year, arg_month, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond, arg_calendar.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new DateTime(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = CalendarError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
  }

  static create_from_codes_in_calendar(arg_era_code, arg_year, arg_month_code, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond, arg_calendar) {
    const buf_arg_era_code = diplomatRuntime.DiplomatBuf.str8(wasm, arg_era_code);
    const buf_arg_month_code = diplomatRuntime.DiplomatBuf.str8(wasm, arg_month_code);
    const diplomat_out = (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
      wasm.ICU4XDateTime_create_from_codes_in_calendar(diplomat_receive_buffer, buf_arg_era_code.ptr, buf_arg_era_code.size, arg_year, buf_arg_month_code.ptr, buf_arg_month_code.size, arg_day, arg_hour, arg_minute, arg_second, arg_nanosecond, arg_calendar.underlying);
      const is_ok = diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4);
      if (is_ok) {
        const ok_value = new DateTime(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), true, []);
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        return ok_value;
      } else {
        const throw_value = CalendarError_rust_to_js[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)];
        wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        throw new diplomatRuntime.FFIError(throw_value);
      }
    })();
    buf_arg_era_code.free();
    buf_arg_month_code.free();
    return diplomat_out;
  }

  static create_from_date_and_time(arg_date, arg_time) {
    return new DateTime(wasm.ICU4XDateTime_create_from_date_and_time(arg_date.underlying, arg_time.underlying), true, []);
  }

  date() {
    return new Date(wasm.ICU4XDateTime_date(this.underlying), true, []);
  }

  time() {
    return new Time(wasm.ICU4XDateTime_time(this.underlying), true, []);
  }

  to_iso() {
    return new IsoDateTime(wasm.ICU4XDateTime_to_iso(this.underlying), true, []);
  }

  to_calendar(arg_calendar) {
    return new DateTime(wasm.ICU4XDateTime_to_calendar(this.underlying, arg_calendar.underlying), true, []);
  }

  hour() {
    return wasm.ICU4XDateTime_hour(this.underlying);
  }

  minute() {
    return wasm.ICU4XDateTime_minute(this.underlying);
  }

  second() {
    return wasm.ICU4XDateTime_second(this.underlying);
  }

  nanosecond() {
    return wasm.ICU4XDateTime_nanosecond(this.underlying);
  }

  day_of_year() {
    return wasm.ICU4XDateTime_day_of_year(this.underlying);
  }

  day_of_month() {
    return wasm.ICU4XDateTime_day_of_month(this.underlying);
  }

  day_of_week() {
    return IsoWeekday_rust_to_js[wasm.ICU4XDateTime_day_of_week(this.underlying)];
  }

  week_of_month(arg_first_weekday) {
    return wasm.ICU4XDateTime_week_of_month(this.underlying, IsoWeekday_js_to_rust[arg_first_weekday]);
  }

  week_of_year(arg_calculator) {
    return (() => {
      const diplomat_receive_buffer = wasm.diplomat_alloc(8, 4);
      wasm.ICU4XDateTime_week_of_year(diplomat_receive_buffer, this.underlying, arg_calculator.underlying);
      const out = new WeekOf(diplomat_receive_buffer);
      wasm.diplomat_free(diplomat_receive_buffer, 8, 4);
      return out;
    })();
  }

  ordinal_month() {
    return wasm.ICU4XDateTime_ordinal_month(this.underlying);
  }

  month_code() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XDateTime_month_code(this.underlying, write);
    });
  }

  year_in_era() {
    return wasm.ICU4XDateTime_year_in_era(this.underlying);
  }

  era() {
    return diplomatRuntime.withDiplomatWrite(wasm, (write) => {
      return wasm.ICU4XDateTime_era(this.underlying, write);
    });
  }

  months_in_year() {
    return wasm.ICU4XDateTime_months_in_year(this.underlying);
  }

  days_in_month() {
    return wasm.ICU4XDateTime_days_in_month(this.underlying);
  }

  days_in_year() {
    return wasm.ICU4XDateTime_days_in_year(this.underlying);
  }

  calendar() {
    return new Calendar(wasm.ICU4XDateTime_calendar(this.underlying), true, []);
  }
}
