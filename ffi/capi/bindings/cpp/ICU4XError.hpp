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
 * 
 * Additional information: [1](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.ParseError.html), [2](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.LimitError.html), [3](https://docs.rs/icu/latest/icu/calendar/struct.RangeError.html), [4](https://docs.rs/icu/latest/icu/calendar/enum.DateError.html), [5](https://docs.rs/icu/latest/icu/datetime/enum.DateTimeError.html), [6](https://docs.rs/icu/latest/icu/locale/enum.ParseError.html), [7](https://docs.rs/icu/latest/icu/provider/struct.DataError.html), [8](https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html), [9](https://docs.rs/icu/latest/icu/timezone/struct.InvalidOffsetError.html), [10](https://docs.rs/icu_experimental/latest/icu_experimental/units/struct.InvalidUnitError.html)
 */
enum struct ICU4XError {

  /**
   * The error is not currently categorized as ICU4XError.
   * Please file a bug
   */
  UnknownError = 0,

  /**
   * Some input was out of bounds
   */
  OutOfBoundsError = 2,
  DataMissingDataKeyError = 256,
  DataMissingVariantError = 257,
  DataMissingLocaleError = 258,
  DataNeedsVariantError = 259,
  DataNeedsLocaleError = 260,
  DataExtraneousLocaleError = 261,
  DataFilteredResourceError = 262,
  DataMismatchedTypeError = 263,
  DataMissingPayloadError = 264,
  DataInvalidStateError = 265,
  DataCustomError = 266,
  DataIoError = 267,
  DataUnavailableBufferFormatError = 268,
  DataMismatchedAnyBufferError = 269,

  /**
   * The subtag being requested was not set
   */
  LocaleUndefinedSubtagError = 512,

  /**
   * The locale or subtag string failed to parse
   */
  LocaleParserLanguageError = 513,
  LocaleParserSubtagError = 514,
  LocaleParserExtensionError = 515,

  /**
   * Attempted to construct an invalid data struct
   */
  DataStructValidityError = 768,
  PropertyUnexpectedPropertyNameError = 1026,
  FixedDecimalLimitError = 1280,
  FixedDecimalSyntaxError = 1281,
  PluralsParseError = 1536,
  CalendarOutOfRangeError = 1795,
  CalendarUnknownEraError = 1796,
  CalendarUnknownMonthCodeError = 1797,
  DateTimePatternError = 2048,
  DateTimeMissingInputFieldError = 2049,
  DateTimeSkeletonError = 2050,
  DateTimeUnsupportedFieldError = 2051,
  DateTimeUnsupportedOptionsError = 2052,
  DateTimeMissingWeekdaySymbolError = 2053,
  DateTimeMissingMonthSymbolError = 2054,
  DateTimeFixedDecimalError = 2055,
  DateTimeMismatchedCalendarError = 2056,
  TimeZoneInvalidOffsetError = 2561,
  TimeZoneMissingInputError = 2562,
  TimeZoneInvalidIdError = 2563,
  InvalidCldrUnitIdentifierError = 3072,
};

#endif
