#ifndef ICU4XFixedDecimalRoundingIncrement_HPP
#define ICU4XFixedDecimalRoundingIncrement_HPP

#include "ICU4XFixedDecimalRoundingIncrement.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalRoundingIncrement.h"


inline capi::ICU4XFixedDecimalRoundingIncrement ICU4XFixedDecimalRoundingIncrement::AsFFI() const {
  return static_cast<capi::ICU4XFixedDecimalRoundingIncrement>(value);
}

inline ICU4XFixedDecimalRoundingIncrement ICU4XFixedDecimalRoundingIncrement::FromFFI(capi::ICU4XFixedDecimalRoundingIncrement c_enum) {
  switch (c_enum) {
    case capi::ICU4XFixedDecimalRoundingIncrement_MultiplesOf1:
    case capi::ICU4XFixedDecimalRoundingIncrement_MultiplesOf2:
    case capi::ICU4XFixedDecimalRoundingIncrement_MultiplesOf5:
    case capi::ICU4XFixedDecimalRoundingIncrement_MultiplesOf25:
      return static_cast<ICU4XFixedDecimalRoundingIncrement::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XFixedDecimalRoundingIncrement_HPP
