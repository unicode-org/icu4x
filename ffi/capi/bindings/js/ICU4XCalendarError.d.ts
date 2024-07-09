
/**

 * Additional information: {@link https://docs.rs/icu/latest/icu/calendar/struct.RangeError.html 1}, {@link https://docs.rs/icu/latest/icu/calendar/enum.DateError.html 2}
 */
export enum ICU4XCalendarError {
  /**
   */
  Unknown = 'Unknown',
  /**
   */
  OutOfRange = 'OutOfRange',
  /**
   */
  UnknownEra = 'UnknownEra',
  /**
   */
  UnknownMonthCode = 'UnknownMonthCode',
}