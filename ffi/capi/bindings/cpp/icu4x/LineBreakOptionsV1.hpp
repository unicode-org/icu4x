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
    /* .strictness = */ strictness.has_value() ? (icu4x::capi::LineBreakStrictness_option{ { strictness.value().AsFFI() }, true }) : (icu4x::capi::LineBreakStrictness_option{ {}, false }),
    /* .word_option = */ word_option.has_value() ? (icu4x::capi::LineBreakWordOption_option{ { word_option.value().AsFFI() }, true }) : (icu4x::capi::LineBreakWordOption_option{ {}, false }),
    /* .ja_zh = */ ja_zh.has_value() ? (diplomat::capi::OptionBool{ { ja_zh.value() }, true }) : (diplomat::capi::OptionBool{ {}, false }),
  };
}

inline icu4x::LineBreakOptionsV1 icu4x::LineBreakOptionsV1::FromFFI(icu4x::capi::LineBreakOptionsV1 c_struct) {
  return icu4x::LineBreakOptionsV1 {
    /* .strictness = */ c_struct.strictness.is_ok ? std::optional(icu4x::LineBreakStrictness::FromFFI(c_struct.strictness.ok)) : std::nullopt,
    /* .word_option = */ c_struct.word_option.is_ok ? std::optional(icu4x::LineBreakWordOption::FromFFI(c_struct.word_option.ok)) : std::nullopt,
    /* .ja_zh = */ c_struct.ja_zh.is_ok ? std::optional(c_struct.ja_zh.ok) : std::nullopt,
  };
}


#endif // icu4x_LineBreakOptionsV1_HPP
