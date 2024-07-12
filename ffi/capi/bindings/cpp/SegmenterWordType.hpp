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


namespace diplomat {
namespace capi {
    extern "C" {
    
    bool ICU4XSegmenterWordType_is_word_like(diplomat::capi::SegmenterWordType self);
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::SegmenterWordType SegmenterWordType::AsFFI() const {
  return static_cast<diplomat::capi::SegmenterWordType>(value);
}

inline SegmenterWordType SegmenterWordType::FromFFI(diplomat::capi::SegmenterWordType c_enum) {
  switch (c_enum) {
    case diplomat::capi::SegmenterWordType_None:
    case diplomat::capi::SegmenterWordType_Number:
    case diplomat::capi::SegmenterWordType_Letter:
      return static_cast<SegmenterWordType::Value>(c_enum);
    default:
      abort();
  }
}

inline bool SegmenterWordType::is_word_like() {
  auto result = diplomat::capi::ICU4XSegmenterWordType_is_word_like(this->AsFFI());
  return result;
}
#endif // SegmenterWordType_HPP
