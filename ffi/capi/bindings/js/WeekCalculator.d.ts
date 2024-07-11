import { u8 } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { IsoWeekday } from "./IsoWeekday";
import { Locale } from "./Locale";
import { WeekendContainsDay } from "./WeekendContainsDay";

/**

 * A Week calculator, useful to be passed in to `week_of_year()` on Date and DateTime types

 * See the {@link https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html Rust documentation for `WeekCalculator`} for more information.
 */
export class WeekCalculator {

  /**

   * Creates a new {@link WeekCalculator `WeekCalculator`} from locale data.

   * See the {@link https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#method.try_new Rust documentation for `try_new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create(provider: DataProvider, locale: Locale): WeekCalculator | never;

  /**

   * Additional information: {@link https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#structfield.first_weekday 1}, {@link https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#structfield.min_week_days 2}
   */
  static create_from_first_day_of_week_and_min_week_days(first_weekday: IsoWeekday, min_week_days: u8): WeekCalculator;

  /**

   * Returns the weekday that starts the week for this object's locale

   * See the {@link https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#structfield.first_weekday Rust documentation for `first_weekday`} for more information.
   */
  first_weekday(): IsoWeekday;

  /**

   * The minimum number of days overlapping a year required for a week to be considered part of that year

   * See the {@link https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#structfield.min_week_days Rust documentation for `min_week_days`} for more information.
   */
  min_week_days(): u8;

  /**

   * See the {@link https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#method.weekend Rust documentation for `weekend`} for more information.
   */
  weekend(): WeekendContainsDay;
}
