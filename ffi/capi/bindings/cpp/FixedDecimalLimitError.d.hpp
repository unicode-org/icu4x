#ifndef FixedDecimalLimitError_D_HPP
#define FixedDecimalLimitError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum FixedDecimalLimitError {
      FixedDecimalLimitError_TodoZst = 0,
    } FixedDecimalLimitError;
}

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

  inline capi::FixedDecimalLimitError AsFFI() const;
  inline static FixedDecimalLimitError FromFFI(capi::FixedDecimalLimitError c_enum);
private:
    Value value;
};


#endif // FixedDecimalLimitError_D_HPP
