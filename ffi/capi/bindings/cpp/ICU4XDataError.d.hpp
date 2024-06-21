#ifndef ICU4XDataError_D_HPP
#define ICU4XDataError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XDataError {
      ICU4XDataError_Unknown = 0,
      ICU4XDataError_MissingDataMarker = 1,
      ICU4XDataError_MissingLocale = 2,
      ICU4XDataError_NeedsLocale = 3,
      ICU4XDataError_ExtraneousLocale = 4,
      ICU4XDataError_FilteredResource = 5,
      ICU4XDataError_MismatchedType = 6,
      ICU4XDataError_Custom = 7,
      ICU4XDataError_Io = 8,
      ICU4XDataError_UnavailableBufferFormat = 9,
      ICU4XDataError_InconsistentData = 10,
    } ICU4XDataError;
}

class ICU4XDataError {
public:
  enum Value {
    Unknown = 0,
    MissingDataMarker = 1,
    MissingLocale = 2,
    NeedsLocale = 3,
    ExtraneousLocale = 4,
    FilteredResource = 5,
    MismatchedType = 6,
    Custom = 7,
    Io = 8,
    UnavailableBufferFormat = 9,
    InconsistentData = 10,
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
