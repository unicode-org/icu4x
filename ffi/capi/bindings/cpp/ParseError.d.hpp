#ifndef ParseError_D_HPP
#define ParseError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum ParseError {
      ParseError_Unknown = 0,
      ParseError_InvalidSyntax = 1,
      ParseError_OutOfRange = 2,
      ParseError_MissingFields = 3,
      ParseError_UnknownCalendar = 4,
    };
} // namespace capi
} // namespace

class ParseError {
public:
  enum Value {
    Unknown = 0,
    InvalidSyntax = 1,
    OutOfRange = 2,
    MissingFields = 3,
    UnknownCalendar = 4,
  };

  ParseError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ParseError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::ParseError AsFFI() const;
  inline static ParseError FromFFI(diplomat::capi::ParseError c_enum);
private:
    Value value;
};


#endif // ParseError_D_HPP
