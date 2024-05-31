#ifndef ICU4XRoundingIncrement_HPP
#define ICU4XRoundingIncrement_HPP

#include "ICU4XRoundingIncrement.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XRoundingIncrement.h"


inline capi::ICU4XRoundingIncrement ICU4XRoundingIncrement::AsFFI() const {
  return static_cast<capi::ICU4XRoundingIncrement>(value);
}

inline ICU4XRoundingIncrement ICU4XRoundingIncrement::FromFFI(capi::ICU4XRoundingIncrement c_enum) {
  switch (c_enum) {
    case capi::ICU4XRoundingIncrement_MultiplesOf1:
    case capi::ICU4XRoundingIncrement_MultiplesOf2:
    case capi::ICU4XRoundingIncrement_MultiplesOf5:
    case capi::ICU4XRoundingIncrement_MultiplesOf25:
      return static_cast<ICU4XRoundingIncrement::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XRoundingIncrement_HPP
