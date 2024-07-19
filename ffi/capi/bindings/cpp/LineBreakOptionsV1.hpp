#ifndef LineBreakOptionsV1_HPP
#define LineBreakOptionsV1_HPP

#include "LineBreakOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "LineBreakStrictness.hpp"
#include "LineBreakWordOption.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::LineBreakOptionsV1 LineBreakOptionsV1::AsFFI() const {
  return diplomat::capi::LineBreakOptionsV1 {
    /* .strictness = */ strictness.AsFFI(),
    /* .word_option = */ word_option.AsFFI(),
    /* .ja_zh = */ ja_zh,
  };
}

inline LineBreakOptionsV1 LineBreakOptionsV1::FromFFI(diplomat::capi::LineBreakOptionsV1 c_struct) {
  return LineBreakOptionsV1 {
    /* .strictness = */ LineBreakStrictness::FromFFI(c_struct.strictness),
    /* .word_option = */ LineBreakWordOption::FromFFI(c_struct.word_option),
    /* .ja_zh = */ c_struct.ja_zh,
  };
}


#endif // LineBreakOptionsV1_HPP
