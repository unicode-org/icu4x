#ifndef LineBreakOptionsV1_D_HPP
#define LineBreakOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "LineBreakStrictness.d.hpp"
#include "LineBreakWordOption.d.hpp"
#include "diplomat_runtime.hpp"

class LineBreakStrictness;
class LineBreakWordOption;


namespace diplomat {
namespace capi {
    struct LineBreakOptionsV1 {
      diplomat::capi::LineBreakStrictness strictness;
      diplomat::capi::LineBreakWordOption word_option;
      bool ja_zh;
    };
} // namespace capi
} // namespace


struct LineBreakOptionsV1 {
  LineBreakStrictness strictness;
  LineBreakWordOption word_option;
  bool ja_zh;

  inline diplomat::capi::LineBreakOptionsV1 AsFFI() const;
  inline static LineBreakOptionsV1 FromFFI(diplomat::capi::LineBreakOptionsV1 c_struct);
};


#endif // LineBreakOptionsV1_D_HPP
