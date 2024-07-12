#ifndef FixedDecimalParseError_D_HPP
#define FixedDecimalParseError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum FixedDecimalParseError {
      FixedDecimalParseError_Unknown = 0,
      FixedDecimalParseError_Limit = 1,
      FixedDecimalParseError_Syntax = 2,
    };
} // namespace capi
} // namespace

class FixedDecimalParseError {
public:
  enum Value {
    Unknown = 0,
    Limit = 1,
    Syntax = 2,
  };

  FixedDecimalParseError() = default;
  // Implicit conversions between enum and ::Value
  constexpr FixedDecimalParseError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::FixedDecimalParseError AsFFI() const;
  inline static FixedDecimalParseError FromFFI(diplomat::capi::FixedDecimalParseError c_enum);
private:
    Value value;
};


#endif // FixedDecimalParseError_D_HPP
