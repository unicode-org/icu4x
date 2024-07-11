#ifndef Error_D_HPP
#define Error_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum Error {
      Error_UnknownError = 0,
      Error_DataMissingDataMarkerError = 256,
      Error_DataMissingLocaleError = 258,
      Error_DataNeedsLocaleError = 260,
      Error_DataExtraneousLocaleError = 261,
      Error_DataFilteredResourceError = 262,
      Error_DataMismatchedTypeError = 263,
      Error_DataCustomError = 266,
      Error_DataIoError = 267,
      Error_DataUnavailableBufferFormatError = 268,
      Error_PropertyUnexpectedPropertyNameError = 1026,
      Error_DateTimePatternError = 2048,
      Error_DateTimeMissingInputFieldError = 2049,
      Error_DateTimeSkeletonError = 2050,
      Error_DateTimeUnsupportedFieldError = 2051,
      Error_DateTimeUnsupportedOptionsError = 2052,
      Error_DateTimeMissingWeekdaySymbolError = 2053,
      Error_DateTimeMissingMonthSymbolError = 2054,
      Error_DateTimeFixedDecimalError = 2055,
      Error_DateTimeMismatchedCalendarError = 2056,
    } Error;
}

class Error {
public:
  enum Value {
    UnknownError = 0,
    DataMissingDataMarkerError = 256,
    DataMissingLocaleError = 258,
    DataNeedsLocaleError = 260,
    DataExtraneousLocaleError = 261,
    DataFilteredResourceError = 262,
    DataMismatchedTypeError = 263,
    DataCustomError = 266,
    DataIoError = 267,
    DataUnavailableBufferFormatError = 268,
    PropertyUnexpectedPropertyNameError = 1026,
    DateTimePatternError = 2048,
    DateTimeMissingInputFieldError = 2049,
    DateTimeSkeletonError = 2050,
    DateTimeUnsupportedFieldError = 2051,
    DateTimeUnsupportedOptionsError = 2052,
    DateTimeMissingWeekdaySymbolError = 2053,
    DateTimeMissingMonthSymbolError = 2054,
    DateTimeFixedDecimalError = 2055,
    DateTimeMismatchedCalendarError = 2056,
  };

  Error() = default;
  // Implicit conversions between enum and ::Value
  constexpr Error(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::Error AsFFI() const;
  inline static Error FromFFI(capi::Error c_enum);
private:
    Value value;
};


#endif // Error_D_HPP
