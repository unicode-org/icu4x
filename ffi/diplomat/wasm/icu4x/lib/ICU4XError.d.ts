
/**

 * A common enum for errors that ICU4X may return, organized by API

 * The error names are stable and can be checked against as strings in the JS API

 * Additional information: {@link https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/enum.Error.html 1}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.DateTimeError.html 2}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/enum.DateTimeFormatterError.html 3}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/locid/enum.ParserError.html 4}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/properties/enum.PropertiesError.html 5}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/enum.PluralRulesError.html 6}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/provider/struct.DataError.html 7}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/provider/enum.DataErrorKind.html 8}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/enum.NormalizerError.html 9}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/enum.TimeZoneError.html 10}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.CollatorError.html 11}, {@link https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/enum.FixedDecimalFormatterError.html 12}
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
  PropertyUnknownScriptIdError = 'PropertyUnknownScriptIdError',
  /**
   */
  PropertyUnknownGeneralCategoryGroupError = 'PropertyUnknownGeneralCategoryGroupError',
  /**
   */
  FixedDecimalLimitError = 'FixedDecimalLimitError',
  /**
   */
  FixedDecimalSyntaxError = 'FixedDecimalSyntaxError',
  /**
   */
  PluralParserError = 'PluralParserError',
  /**
   */
  DateTimeParseError = 'DateTimeParseError',
  /**
   */
  DateTimeOverflowError = 'DateTimeOverflowError',
  /**
   */
  DateTimeUnderflowError = 'DateTimeUnderflowError',
  /**
   */
  DateTimeOutOfRangeError = 'DateTimeOutOfRangeError',
  /**
   */
  DateTimeUnknownEraError = 'DateTimeUnknownEraError',
  /**
   */
  DateTimeUnknownMonthCodeError = 'DateTimeUnknownMonthCodeError',
  /**
   */
  DateTimeMissingInputError = 'DateTimeMissingInputError',
  /**
   */
  DateTimeUnknownAnyCalendarKindError = 'DateTimeUnknownAnyCalendarKindError',
  /**
   */
  DateTimeFormatPatternError = 'DateTimeFormatPatternError',
  /**
   */
  DateTimeFormatMissingInputFieldError = 'DateTimeFormatMissingInputFieldError',
  /**
   */
  DateTimeFormatSkeletonError = 'DateTimeFormatSkeletonError',
  /**
   */
  DateTimeFormatUnsupportedFieldError = 'DateTimeFormatUnsupportedFieldError',
  /**
   */
  DateTimeFormatUnsupportedOptionsError = 'DateTimeFormatUnsupportedOptionsError',
  /**
   */
  DateTimeFormatMissingWeekdaySymbolError = 'DateTimeFormatMissingWeekdaySymbolError',
  /**
   */
  DateTimeFormatMissingMonthSymbolError = 'DateTimeFormatMissingMonthSymbolError',
  /**
   */
  DateTimeFormatFixedDecimalError = 'DateTimeFormatFixedDecimalError',
  /**
   */
  DateTimeFormatMismatchedAnyCalendarError = 'DateTimeFormatMismatchedAnyCalendarError',
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
  TimeZoneOffsetOutOfBoundsError = 'TimeZoneOffsetOutOfBoundsError',
  /**
   */
  TimeZoneInvalidOffsetError = 'TimeZoneInvalidOffsetError',
  /**
   */
  TimeZoneMissingInputError = 'TimeZoneMissingInputError',
  /**
   */
  NormalizerFutureExtensionError = 'NormalizerFutureExtensionError',
  /**
   */
  NormalizerValidationError = 'NormalizerValidationError',
}