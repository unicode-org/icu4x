#ifndef LineBreakOptionsV1_HPP
#define LineBreakOptionsV1_HPP

#include "LineBreakOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "LineBreakStrictness.hpp"
#include "LineBreakWordOption.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::LineBreakOptionsV1 LineBreakOptionsV1::AsFFI() const {
  return capi::LineBreakOptionsV1 {
    .strictness = strictness.AsFFI(),
    .word_option = word_option.AsFFI(),
    .ja_zh = ja_zh,
  };
}

inline LineBreakOptionsV1 LineBreakOptionsV1::FromFFI(capi::LineBreakOptionsV1 c_struct) {
  return LineBreakOptionsV1 {
    .strictness = LineBreakStrictness::FromFFI(c_struct.strictness),
    .word_option = LineBreakWordOption::FromFFI(c_struct.word_option),
    .ja_zh = c_struct.ja_zh,
  };
}


#endif // LineBreakOptionsV1_HPP
