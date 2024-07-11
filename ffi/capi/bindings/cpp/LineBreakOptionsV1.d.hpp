#ifndef LineBreakOptionsV1_D_HPP
#define LineBreakOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "LineBreakStrictness.d.hpp"
#include "LineBreakWordOption.d.hpp"

class LineBreakStrictness;
class LineBreakWordOption;


namespace capi {
    typedef struct LineBreakOptionsV1 {
      LineBreakStrictness strictness;
      LineBreakWordOption word_option;
      bool ja_zh;
    } LineBreakOptionsV1;
}

struct LineBreakOptionsV1 {
  LineBreakStrictness strictness;
  LineBreakWordOption word_option;
  bool ja_zh;

  inline capi::LineBreakOptionsV1 AsFFI() const;
  inline static LineBreakOptionsV1 FromFFI(capi::LineBreakOptionsV1 c_struct);
};


#endif // LineBreakOptionsV1_D_HPP
