#ifndef FromIxdtfError_D_HPP
#define FromIxdtfError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum FromIxdtfError {
      FromIxdtfError_Unknown = 0,
      FromIxdtfError_InvalidSyntax = 1,
      FromIxdtfError_OutOfRange = 2,
      FromIxdtfError_MissingFields = 3,
      FromIxdtfError_UnknownCalendar = 4,
    };
} // namespace capi
} // namespace

class FromIxdtfError {
public:
  enum Value {
    Unknown = 0,
    InvalidSyntax = 1,
    OutOfRange = 2,
    MissingFields = 3,
    UnknownCalendar = 4,
  };

  FromIxdtfError() = default;
  // Implicit conversions between enum and ::Value
  constexpr FromIxdtfError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::FromIxdtfError AsFFI() const;
  inline static FromIxdtfError FromFFI(diplomat::capi::FromIxdtfError c_enum);
private:
    Value value;
};


#endif // FromIxdtfError_D_HPP
