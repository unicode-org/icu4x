import { FFIError } from "./diplomat-runtime"
import { AnyCalendarKind } from "./AnyCalendarKind";
import { DataError } from "./DataError";
import { DataProvider } from "./DataProvider";
import { Locale } from "./Locale";

/**

 * See the {@link https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html Rust documentation for `AnyCalendar`} for more information.
 */
export class Calendar {

  /**

   * Creates a new {@link Calendar `Calendar`} from the specified date and time.

   * See the {@link https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.new_for_locale Rust documentation for `new_for_locale`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_for_locale(provider: DataProvider, locale: Locale): Calendar | never;

  /**

   * Creates a new {@link Calendar `Calendar`} from the specified date and time.

   * See the {@link https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.new Rust documentation for `new`} for more information.
   * @throws {@link FFIError}<{@link DataError}>
   */
  static create_for_kind(provider: DataProvider, kind: AnyCalendarKind): Calendar | never;

  /**

   * Returns the kind of this calendar

   * See the {@link https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.kind Rust documentation for `kind`} for more information.
   */
  kind(): AnyCalendarKind;
}
