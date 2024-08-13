#ifndef icu4x_LineBreakOptionsV1_HPP
#define icu4x_LineBreakOptionsV1_HPP

#include "LineBreakOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "LineBreakStrictness.hpp"
#include "LineBreakWordOption.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline icu4x::capi::LineBreakOptionsV1 icu4x::LineBreakOptionsV1::AsFFI() const {
  return icu4x::capi::LineBreakOptionsV1 {
    /* .strictness = */ strictness.AsFFI(),
    /* .word_option = */ word_option.AsFFI(),
    /* .ja_zh = */ ja_zh,
  };
}

inline icu4x::LineBreakOptionsV1 icu4x::LineBreakOptionsV1::FromFFI(icu4x::capi::LineBreakOptionsV1 c_struct) {
  return icu4x::LineBreakOptionsV1 {
    /* .strictness = */ icu4x::LineBreakStrictness::FromFFI(c_struct.strictness),
    /* .word_option = */ icu4x::LineBreakWordOption::FromFFI(c_struct.word_option),
    /* .ja_zh = */ c_struct.ja_zh,
  };
}


#endif // icu4x_LineBreakOptionsV1_HPP
