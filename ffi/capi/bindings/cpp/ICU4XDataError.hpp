#ifndef ICU4XDataError_HPP
#define ICU4XDataError_HPP

#include "ICU4XDataError.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.h"


inline capi::ICU4XDataError ICU4XDataError::AsFFI() const {
  return static_cast<capi::ICU4XDataError>(value);
}

inline ICU4XDataError ICU4XDataError::FromFFI(capi::ICU4XDataError c_enum) {
  switch (c_enum) {
    case capi::ICU4XDataError_Unknown:
    case capi::ICU4XDataError_MissingDataMarker:
    case capi::ICU4XDataError_MissingLocale:
    case capi::ICU4XDataError_NeedsLocale:
    case capi::ICU4XDataError_ExtraneousLocale:
    case capi::ICU4XDataError_FilteredResource:
    case capi::ICU4XDataError_MismatchedType:
    case capi::ICU4XDataError_Custom:
    case capi::ICU4XDataError_Io:
    case capi::ICU4XDataError_UnavailableBufferFormat:
    case capi::ICU4XDataError_InconsistentData:
      return static_cast<ICU4XDataError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XDataError_HPP
