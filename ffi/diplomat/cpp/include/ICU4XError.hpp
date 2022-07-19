#ifndef ICU4XError_HPP
#define ICU4XError_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XError.h"



/**
 * A common enum for errors that ICU4X may return, organized by API
 * 
 * The error names are stable and can be checked against as strings in the JS API
 */
enum struct ICU4XError {

  /**
   * The error is not currently categorized as ICU4XError.
   * Please file a bug
   */
  UnknownError = 0,

  /**
   * An error arising from writing to a string
   * Typically found when not enough space is allocated
   * Most APIs that return a string may return this error
   */
  WriteableError = 1,
  OutOfBoundsError = 2,
  DataMissingDataKeyError = 256,
  DataMissingVariantError = 257,
  DataMissingLocaleError = 258,
  DataMissingDataOptionsError = 259,
  DataNeedsVariantError = 260,
  DataNeedsLocaleError = 261,
  DataExtraneousDataOptionsError = 262,
  DataFilteredResourceError = 263,
  DataMismatchedTypeError = 264,
  DataMissingPayloadError = 265,
  DataInvalidStateError = 266,
  DataCustomError = 267,
  DataIoError = 268,
  DataUnavailableBufferFormatError = 269,

  /**
   * The subtag being requested was not set
   */
  LocaleUndefinedSubtagError = 512,

  /**
   * The locale or subtag string failed to parse
   */
  LocaleParserError = 513,

  /**
   * Attempted to construct an invalid data struct
   */
  DataStructValidityError = 768,
  PropertyUnknownScriptIdError = 1024,
  PropertyUnknownGeneralCategoryGroupError = 1025,
  DecimalLimitError = 1280,
  DecimalSyntaxError = 1281,
  PluralParserError = 1536,
  DateTimeParseError = 1792,
  DateTimeOverflowError = 1793,
  DateTimeUnderflowError = 1794,
  DateTimeInvalidTimeZoneOffsetError = 1795,
  DateTimeOutOfRangeError = 1796,
  DateTimeMissingInputError = 1797,
  DateTimeFormatPatternError = 2048,
  DateTimeFormatMissingInputFieldError = 2049,
  DateTimeFormatSkeletonError = 2050,
  DateTimeFormatUnsupportedFieldError = 2051,
  DateTimeFormatUnsupportedOptionsError = 2052,
  DateTimeFormatMissingWeekdaySymbolError = 2053,
  DateTimeFormatMissingMonthSymbolError = 2054,
  DateTimeFormatFixedDecimalError = 2055,
  DateTimeFormatMismatchedAnyCalendarError = 2056,
  DateTimeFormatMismatchedCalendarLocaleError = 2057,
};

#endif
