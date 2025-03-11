#ifndef icu4x_DateTimeWriteError_D_HPP
#define icu4x_DateTimeWriteError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum DateTimeWriteError {
      DateTimeWriteError_Unknown = 0,
      DateTimeWriteError_InvalidMonthCode = 2,
      DateTimeWriteError_InvalidEra = 3,
      DateTimeWriteError_InvalidCyclicYear = 4,
      DateTimeWriteError_DecimalFormatterNotLoaded = 5,
      DateTimeWriteError_NamesNotLoaded = 6,
      DateTimeWriteError_MissingInputField = 7,
      DateTimeWriteError_UnsupportedLength = 8,
      DateTimeWriteError_UnsupportedField = 9,
    };
    
    typedef struct DateTimeWriteError_option {union { DateTimeWriteError ok; }; bool is_ok; } DateTimeWriteError_option;
} // namespace capi
} // namespace

namespace icu4x {
class DateTimeWriteError {
public:
  enum Value {
    Unknown = 0,
    InvalidMonthCode = 2,
    InvalidEra = 3,
    InvalidCyclicYear = 4,
    DecimalFormatterNotLoaded = 5,
    NamesNotLoaded = 6,
    MissingInputField = 7,
    UnsupportedLength = 8,
    UnsupportedField = 9,
  };

  DateTimeWriteError() = default;
  // Implicit conversions between enum and ::Value
  constexpr DateTimeWriteError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::DateTimeWriteError AsFFI() const;
  inline static icu4x::DateTimeWriteError FromFFI(icu4x::capi::DateTimeWriteError c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_DateTimeWriteError_D_HPP
