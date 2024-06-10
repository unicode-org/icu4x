
/**

 * A common enum for errors that ICU4X may return, organized by API

 * The error names are stable and can be checked against as strings in the JS API

 * Additional information: {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.ParseError.html 1}, {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.LimitError.html 2}, {@link https://docs.rs/icu/latest/icu/calendar/struct.RangeError.html 3}, {@link https://docs.rs/icu/latest/icu/calendar/enum.DateError.html 4}, {@link https://docs.rs/icu/latest/icu/datetime/enum.DateTimeError.html 5}, {@link https://docs.rs/icu/latest/icu/locale/enum.ParseError.html 6}, {@link https://docs.rs/icu/latest/icu/provider/struct.DataError.html 7}, {@link https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html 8}, {@link https://docs.rs/icu/latest/icu/timezone/struct.InvalidOffsetError.html 9}, {@link https://docs.rs/icu_experimental/latest/icu_experimental/units/struct.InvalidUnitError.html 10}
 */
export enum ICU4XError {
  /**

   * The error is not currently categorized as ICU4XError. Please file a bug
   */
  UnknownError = 'UnknownError',
  /**
   */
  DataMissingDataMarkerError = 'DataMissingDataMarkerError',
  /**
   */
  DataMissingVariantError = 'DataMissingVariantError',
  /**
   */
  DataMissingLocaleError = 'DataMissingLocaleError',
  /**
   */
  DataNeedsVariantError = 'DataNeedsVariantError',
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
  DataMissingPayloadError = 'DataMissingPayloadError',
  /**
   */
  DataInvalidStateError = 'DataInvalidStateError',
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
  DataMismatchedAnyBufferError = 'DataMismatchedAnyBufferError',
  /**

   * The locale or subtag string failed to parse
   */
  LocaleParserLanguageError = 'LocaleParserLanguageError',
  /**
   */
  LocaleParserSubtagError = 'LocaleParserSubtagError',
  /**
   */
  LocaleParserExtensionError = 'LocaleParserExtensionError',
  /**

   * Attempted to construct an invalid data struct
   */
  DataStructValidityError = 'DataStructValidityError',
  /**
   */
  PropertyUnexpectedPropertyNameError = 'PropertyUnexpectedPropertyNameError',
  /**
   */
  FixedDecimalLimitError = 'FixedDecimalLimitError',
  /**
   */
  FixedDecimalSyntaxError = 'FixedDecimalSyntaxError',
  /**
   */
  PluralsParseError = 'PluralsParseError',
  /**
   */
  CalendarOutOfRangeError = 'CalendarOutOfRangeError',
  /**
   */
  CalendarUnknownEraError = 'CalendarUnknownEraError',
  /**
   */
  CalendarUnknownMonthCodeError = 'CalendarUnknownMonthCodeError',
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
  /**
   */
  TimeZoneInvalidOffsetError = 'TimeZoneInvalidOffsetError',
  /**
   */
  TimeZoneInvalidIdError = 'TimeZoneInvalidIdError',
}