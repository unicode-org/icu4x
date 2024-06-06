#ifndef ICU4XError_D_HPP
#define ICU4XError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.h"


class ICU4XError {
public:
  enum Value {
    UnknownError = 0,
    DataMissingDataMarkerError = 256,
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
    LocaleParserLanguageError = 513,
    LocaleParserSubtagError = 514,
    LocaleParserExtensionError = 515,
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
    TimeZoneInvalidIdError = 2563,
  };

  ICU4XError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XError AsFFI() const;
  inline static ICU4XError FromFFI(capi::ICU4XError c_enum);
private:
    Value value;
};


#endif // ICU4XError_D_HPP
