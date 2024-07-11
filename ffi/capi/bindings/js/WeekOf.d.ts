import { u16 } from "./diplomat-runtime"
import { WeekRelativeUnit } from "./WeekRelativeUnit";

/**

 * See the {@link https://docs.rs/icu/latest/icu/calendar/week/struct.WeekOf.html Rust documentation for `WeekOf`} for more information.
 */
export class WeekOf {
  week: u16;
  unit: WeekRelativeUnit;
}
