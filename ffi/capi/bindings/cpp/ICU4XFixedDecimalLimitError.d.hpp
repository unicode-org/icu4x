#ifndef ICU4XFixedDecimalLimitError_D_HPP
#define ICU4XFixedDecimalLimitError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XFixedDecimalLimitError {
      ICU4XFixedDecimalLimitError_TodoZst = 0,
    } ICU4XFixedDecimalLimitError;
}

class ICU4XFixedDecimalLimitError {
public:
  enum Value {
    TodoZst = 0,
  };

  ICU4XFixedDecimalLimitError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XFixedDecimalLimitError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XFixedDecimalLimitError AsFFI() const;
  inline static ICU4XFixedDecimalLimitError FromFFI(capi::ICU4XFixedDecimalLimitError c_enum);
private:
    Value value;
};


#endif // ICU4XFixedDecimalLimitError_D_HPP
