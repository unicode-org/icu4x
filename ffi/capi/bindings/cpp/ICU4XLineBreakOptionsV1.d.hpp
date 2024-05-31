#ifndef ICU4XLineBreakOptionsV1_D_HPP
#define ICU4XLineBreakOptionsV1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLineBreakOptionsV1.d.h"
#include "ICU4XLineBreakStrictness.d.hpp"
#include "ICU4XLineBreakWordOption.d.hpp"

class ICU4XLineBreakStrictness;
class ICU4XLineBreakWordOption;


struct ICU4XLineBreakOptionsV1 {
  ICU4XLineBreakStrictness strictness;
  ICU4XLineBreakWordOption word_option;
  bool ja_zh;

  inline capi::ICU4XLineBreakOptionsV1 AsFFI() const;
  inline static ICU4XLineBreakOptionsV1 FromFFI(capi::ICU4XLineBreakOptionsV1 c_struct);
};


#endif // ICU4XLineBreakOptionsV1_D_HPP
