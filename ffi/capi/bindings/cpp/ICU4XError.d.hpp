#ifndef ICU4XError_D_HPP
#define ICU4XError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XError {
      ICU4XError_UnknownError = 0,
      ICU4XError_DataMissingDataMarkerError = 256,
      ICU4XError_DataMissingLocaleError = 258,
      ICU4XError_DataNeedsLocaleError = 260,
      ICU4XError_DataExtraneousLocaleError = 261,
      ICU4XError_DataFilteredResourceError = 262,
      ICU4XError_DataMismatchedTypeError = 263,
      ICU4XError_DataCustomError = 266,
      ICU4XError_DataIoError = 267,
      ICU4XError_DataUnavailableBufferFormatError = 268,
      ICU4XError_PropertyUnexpectedPropertyNameError = 1026,
      ICU4XError_DateTimePatternError = 2048,
      ICU4XError_DateTimeMissingInputFieldError = 2049,
      ICU4XError_DateTimeSkeletonError = 2050,
      ICU4XError_DateTimeUnsupportedFieldError = 2051,
      ICU4XError_DateTimeUnsupportedOptionsError = 2052,
      ICU4XError_DateTimeMissingWeekdaySymbolError = 2053,
      ICU4XError_DateTimeMissingMonthSymbolError = 2054,
      ICU4XError_DateTimeFixedDecimalError = 2055,
      ICU4XError_DateTimeMismatchedCalendarError = 2056,
    } ICU4XError;
}

class ICU4XError {
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
