
/**

 * A common enum for errors that ICU4X may return, organized by API

 * The error names are stable and can be checked against as strings in the JS API
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

   * The subtag being requested was not set
   */
  LocaleUndefinedSubtagError = 'LocaleUndefinedSubtagError',
  /**

   * The locale or subtag string failed to parse
   */
  LocaleParserError = 'LocaleParserError',
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
  DecimalLimitError = 'DecimalLimitError',
  /**
   */
  DecimalSyntaxError = 'DecimalSyntaxError',
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
  DateTimeInvalidTimeZoneOffsetError = 'DateTimeInvalidTimeZoneOffsetError',
  /**
   */
  DateTimeOutOfRangeError = 'DateTimeOutOfRangeError',
  /**
   */
  DateTimeMissingInputError = 'DateTimeMissingInputError',
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
  DateTimeFormatMismatchedCalendarLocaleError = 'DateTimeFormatMismatchedCalendarLocaleError',
}