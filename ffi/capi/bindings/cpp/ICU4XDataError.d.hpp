#ifndef ICU4XDataError_D_HPP
#define ICU4XDataError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.h"


class ICU4XDataError {
public:
  enum Value {
    Unknown = 0,
    MissingDataMarker = 1,
    MissingVariant = 2,
    MissingLocale = 3,
    NeedsVariant = 4,
    NeedsLocale = 5,
    ExtraneousLocale = 6,
    FilteredResource = 7,
    MismatchedType = 8,
    InvalidState = 9,
    Custom = 10,
    Io = 11,
    UnavailableBufferFormat = 12,
    MismatchedAnyBuffer = 13,
    DataStructValidityError = 14,
  };

  ICU4XDataError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XDataError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XDataError AsFFI() const;
  inline static ICU4XDataError FromFFI(capi::ICU4XDataError c_enum);
private:
    Value value;
};


#endif // ICU4XDataError_D_HPP
