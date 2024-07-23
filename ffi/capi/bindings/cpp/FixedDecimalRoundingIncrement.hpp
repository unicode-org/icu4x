#ifndef FixedDecimalRoundingIncrement_HPP
#define FixedDecimalRoundingIncrement_HPP

#include "FixedDecimalRoundingIncrement.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::FixedDecimalRoundingIncrement FixedDecimalRoundingIncrement::AsFFI() const {
  return static_cast<diplomat::capi::FixedDecimalRoundingIncrement>(value);
}

inline FixedDecimalRoundingIncrement FixedDecimalRoundingIncrement::FromFFI(diplomat::capi::FixedDecimalRoundingIncrement c_enum) {
  switch (c_enum) {
    case diplomat::capi::FixedDecimalRoundingIncrement_MultiplesOf1:
    case diplomat::capi::FixedDecimalRoundingIncrement_MultiplesOf2:
    case diplomat::capi::FixedDecimalRoundingIncrement_MultiplesOf5:
    case diplomat::capi::FixedDecimalRoundingIncrement_MultiplesOf25:
      return static_cast<FixedDecimalRoundingIncrement::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalRoundingIncrement_HPP
