#ifndef ICU4XTransformResult_D_HPP
#define ICU4XTransformResult_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XTransformResult.d.h"


class ICU4XTransformResult {
public:
  enum Value {
    Modified = 0,
    Unmodified = 1,
  };

  ICU4XTransformResult() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XTransformResult(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XTransformResult AsFFI() const;
  inline static ICU4XTransformResult FromFFI(capi::ICU4XTransformResult c_enum);
private:
    Value value;
};


#endif // ICU4XTransformResult_D_HPP
