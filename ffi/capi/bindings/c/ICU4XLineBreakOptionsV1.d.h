#ifndef ICU4XLineBreakOptionsV1_D_H
#define ICU4XLineBreakOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XLineBreakStrictness.d.h"
#include "ICU4XLineBreakWordOption.d.h"




typedef struct ICU4XLineBreakOptionsV1 {
  ICU4XLineBreakStrictness strictness;
  ICU4XLineBreakWordOption word_option;
  bool ja_zh;
} ICU4XLineBreakOptionsV1;





#endif // ICU4XLineBreakOptionsV1_D_H
