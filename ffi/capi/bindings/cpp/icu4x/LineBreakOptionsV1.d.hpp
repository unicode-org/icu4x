#ifndef icu4x_LineBreakOptionsV1_D_HPP
#define icu4x_LineBreakOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "LineBreakStrictness.d.hpp"
#include "LineBreakWordOption.d.hpp"

namespace icu4x {
class LineBreakStrictness;
class LineBreakWordOption;
}


namespace icu4x {
namespace capi {
    struct LineBreakOptionsV1 {
      icu4x::capi::LineBreakStrictness strictness;
      icu4x::capi::LineBreakWordOption word_option;
      bool ja_zh;
    };
    
    typedef struct LineBreakOptionsV1_option {union { LineBreakOptionsV1 ok; }; bool is_ok; } LineBreakOptionsV1_option;
} // namespace capi
} // namespace


namespace icu4x {
struct LineBreakOptionsV1 {
  icu4x::LineBreakStrictness strictness;
  icu4x::LineBreakWordOption word_option;
  bool ja_zh;

  inline icu4x::capi::LineBreakOptionsV1 AsFFI() const;
  inline static icu4x::LineBreakOptionsV1 FromFFI(icu4x::capi::LineBreakOptionsV1 c_struct);
};

} // namespace
#endif // icu4x_LineBreakOptionsV1_D_HPP
