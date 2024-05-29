#ifndef ICU4XPluralsParseError_HPP
#define ICU4XPluralsParseError_HPP

#include "ICU4XPluralsParseError.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XPluralsParseError.h"


inline capi::ICU4XPluralsParseError ICU4XPluralsParseError::AsFFI() const {
  return static_cast<capi::ICU4XPluralsParseError>(value);
}

inline ICU4XPluralsParseError ICU4XPluralsParseError::FromFFI(capi::ICU4XPluralsParseError c_enum) {
  switch (c_enum) {
    case capi::ICU4XPluralsParseError_TodoZst:
      return static_cast<ICU4XPluralsParseError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XPluralsParseError_HPP
