
/**

 * Legacy error

 * Additional information: {@link https://docs.rs/icu/latest/icu/datetime/enum.DateTimeError.html 1}, {@link https://docs.rs/icu/latest/icu/provider/struct.DataError.html 2}, {@link https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html 3}
 */
export enum Error {
  /**

   * The error is not currently categorized as Error. Please file a bug
   */
  UnknownError = 'UnknownError',
  /**
   */
  DataMissingDataMarkerError = 'DataMissingDataMarkerError',
  /**
   */
  DataMissingLocaleError = 'DataMissingLocaleError',
  /**
   */
  DataNeedsLocaleError = 'DataNeedsLocaleError',
  /**
   */
  DataExtraneousLocaleError = 'DataExtraneousLocaleError',
  /**
   */
  DataFilteredResourceError = 'DataFilteredResourceError',
  /**
   */
  DataMismatchedTypeError = 'DataMismatchedTypeError',
  /**
   */
  DataCustomError = 'DataCustomError',
  /**
   */
  DataIoError = 'DataIoError',
  /**
   */
  DataUnavailableBufferFormatError = 'DataUnavailableBufferFormatError',
  /**
   */
  PropertyUnexpectedPropertyNameError = 'PropertyUnexpectedPropertyNameError',
  /**
   */
  DateTimePatternError = 'DateTimePatternError',
  /**
   */
  DateTimeMissingInputFieldError = 'DateTimeMissingInputFieldError',
  /**
   */
  DateTimeSkeletonError = 'DateTimeSkeletonError',
  /**
   */
  DateTimeUnsupportedFieldError = 'DateTimeUnsupportedFieldError',
  /**
   */
  DateTimeUnsupportedOptionsError = 'DateTimeUnsupportedOptionsError',
  /**
   */
  DateTimeMissingWeekdaySymbolError = 'DateTimeMissingWeekdaySymbolError',
  /**
   */
  DateTimeMissingMonthSymbolError = 'DateTimeMissingMonthSymbolError',
  /**
   */
  DateTimeFixedDecimalError = 'DateTimeFixedDecimalError',
  /**
   */
  DateTimeMismatchedCalendarError = 'DateTimeMismatchedCalendarError',
}