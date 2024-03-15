
/**

 * A common enum for errors that ICU4X may return, organized by API

 * The error names are stable and can be checked against as strings in the JS API

 * Additional information: {@link https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.FixedDecimalError.html 1}, {@link https://docs.rs/icu/latest/icu/calendar/enum.CalendarError.html 2}, {@link https://docs.rs/icu/latest/icu/datetime/enum.DateTimeError.html 3}, {@link https://docs.rs/icu/latest/icu/locid/enum.ParseError.html 4}, {@link https://docs.rs/icu/latest/icu/provider/struct.DataError.html 5}, {@link https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html 6}, {@link https://docs.rs/icu/latest/icu/timezone/struct.InvalidOffsetError.html 7}
 */
export enum ICU4XError {
  /**

   * The error is not currently categorized as ICU4XError. Please file a bug
   */
  UnknownError = 'UnknownError',
  /**

   * An error arising from writing to a string Typically found when not enough space is allocated Most APIs that return a string may return this error
   */
  WriteableError = 'WriteableError',
  /**
   */
  OutOfBoundsError = 'OutOfBoundsError',
  /**
   */
  DataMissingDataKeyError = 'DataMissingDataKeyError',
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

   * The subtag being requested was not set
   */
  LocaleUndefinedSubtagError = 'LocaleUndefinedSubtagError',
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
  CalendarParseError = 'CalendarParseError',
  /**
   */
  CalendarOverflowError = 'CalendarOverflowError',
  /**
   */
  CalendarUnderflowError = 'CalendarUnderflowError',
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
  CalendarMissingInputError = 'CalendarMissingInputError',
  /**
   */
  CalendarUnknownKindError = 'CalendarUnknownKindError',
  /**
   */
  CalendarMissingError = 'CalendarMissingError',
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
  TinyStrTooLargeError = 'TinyStrTooLargeError',
  /**
   */
  TinyStrContainsNullError = 'TinyStrContainsNullError',
  /**
   */
  TinyStrNonAsciiError = 'TinyStrNonAsciiError',
  /**
   */
  TimeZoneInvalidOffsetError = 'TimeZoneInvalidOffsetError',
  /**
   */
  TimeZoneMissingInputError = 'TimeZoneMissingInputError',
  /**
   */
  TimeZoneInvalidIdError = 'TimeZoneInvalidIdError',
}