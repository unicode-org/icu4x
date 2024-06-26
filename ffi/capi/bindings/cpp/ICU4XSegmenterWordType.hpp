#ifndef ICU4XSegmenterWordType_HPP
#define ICU4XSegmenterWordType_HPP

#include "ICU4XSegmenterWordType.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    bool ICU4XSegmenterWordType_is_word_like(ICU4XSegmenterWordType self);
    
    
    } // extern "C"
}


inline capi::ICU4XSegmenterWordType ICU4XSegmenterWordType::AsFFI() const {
  return static_cast<capi::ICU4XSegmenterWordType>(value);
}

inline ICU4XSegmenterWordType ICU4XSegmenterWordType::FromFFI(capi::ICU4XSegmenterWordType c_enum) {
  switch (c_enum) {
    case capi::ICU4XSegmenterWordType_None:
    case capi::ICU4XSegmenterWordType_Number:
    case capi::ICU4XSegmenterWordType_Letter:
      return static_cast<ICU4XSegmenterWordType::Value>(c_enum);
    default:
      abort();
  }
}

inline bool ICU4XSegmenterWordType::is_word_like() {
  auto result = capi::ICU4XSegmenterWordType_is_word_like(this->AsFFI());
  return result;
}
#endif // ICU4XSegmenterWordType_HPP
