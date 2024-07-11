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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::FixedDecimalRoundingIncrement FixedDecimalRoundingIncrement::AsFFI() const {
  return static_cast<capi::FixedDecimalRoundingIncrement>(value);
}

inline FixedDecimalRoundingIncrement FixedDecimalRoundingIncrement::FromFFI(capi::FixedDecimalRoundingIncrement c_enum) {
  switch (c_enum) {
    case capi::FixedDecimalRoundingIncrement_MultiplesOf1:
    case capi::FixedDecimalRoundingIncrement_MultiplesOf2:
    case capi::FixedDecimalRoundingIncrement_MultiplesOf5:
    case capi::FixedDecimalRoundingIncrement_MultiplesOf25:
      return static_cast<FixedDecimalRoundingIncrement::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalRoundingIncrement_HPP
