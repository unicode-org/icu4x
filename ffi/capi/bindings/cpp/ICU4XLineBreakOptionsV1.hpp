#ifndef ICU4XLineBreakOptionsV1_HPP
#define ICU4XLineBreakOptionsV1_HPP

#include "ICU4XLineBreakOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLineBreakOptionsV1.h"
#include "ICU4XLineBreakStrictness.hpp"
#include "ICU4XLineBreakWordOption.hpp"



inline capi::ICU4XLineBreakOptionsV1 ICU4XLineBreakOptionsV1::AsFFI() const {
  return capi::ICU4XLineBreakOptionsV1 {
    .strictness = strictness.AsFFI(),
    .word_option = word_option.AsFFI(),
    .ja_zh = ja_zh,
  };
}

inline ICU4XLineBreakOptionsV1 ICU4XLineBreakOptionsV1::FromFFI(capi::ICU4XLineBreakOptionsV1 c_struct) {
  return ICU4XLineBreakOptionsV1 {
    .strictness = ICU4XLineBreakStrictness::FromFFI(c_struct.strictness),
    .word_option = ICU4XLineBreakWordOption::FromFFI(c_struct.word_option),
    .ja_zh = c_struct.ja_zh,
  };
}


#endif // ICU4XLineBreakOptionsV1_HPP
