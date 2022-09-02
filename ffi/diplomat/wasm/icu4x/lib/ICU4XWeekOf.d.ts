import { u16 } from "./diplomat-runtime"
import { ICU4XWeekRelativeUnit } from "./ICU4XWeekRelativeUnit";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/week/struct.WeekOf.html Rust documentation for `WeekOf`} for more information.
 */
export class ICU4XWeekOf {
  week: u16;
  unit: ICU4XWeekRelativeUnit;
}
