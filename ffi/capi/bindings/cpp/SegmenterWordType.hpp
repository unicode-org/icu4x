#ifndef SegmenterWordType_HPP
#define SegmenterWordType_HPP

#include "SegmenterWordType.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    bool ICU4XSegmenterWordType_is_word_like(SegmenterWordType self);
    
    
    } // extern "C"
}


inline capi::SegmenterWordType SegmenterWordType::AsFFI() const {
  return static_cast<capi::SegmenterWordType>(value);
}

inline SegmenterWordType SegmenterWordType::FromFFI(capi::SegmenterWordType c_enum) {
  switch (c_enum) {
    case capi::SegmenterWordType_None:
    case capi::SegmenterWordType_Number:
    case capi::SegmenterWordType_Letter:
      return static_cast<SegmenterWordType::Value>(c_enum);
    default:
      abort();
  }
}

inline bool SegmenterWordType::is_word_like() {
  auto result = capi::ICU4XSegmenterWordType_is_word_like(this->AsFFI());
  return result;
}
#endif // SegmenterWordType_HPP
