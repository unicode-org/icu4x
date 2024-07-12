#ifndef FixedDecimalLimitError_D_HPP
#define FixedDecimalLimitError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum FixedDecimalLimitError {
      FixedDecimalLimitError_TodoZst = 0,
    };
} // namespace capi
} // namespace

class FixedDecimalLimitError {
public:
  enum Value {
    TodoZst = 0,
  };

  FixedDecimalLimitError() = default;
  // Implicit conversions between enum and ::Value
  constexpr FixedDecimalLimitError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::FixedDecimalLimitError AsFFI() const;
  inline static FixedDecimalLimitError FromFFI(diplomat::capi::FixedDecimalLimitError c_enum);
private:
    Value value;
};


#endif // FixedDecimalLimitError_D_HPP
