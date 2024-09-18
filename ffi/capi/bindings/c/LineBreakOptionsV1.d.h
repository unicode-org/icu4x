#ifndef LineBreakOptionsV1_D_H
#define LineBreakOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "LineBreakStrictness.d.h"
#include "LineBreakWordOption.d.h"




typedef struct LineBreakOptionsV1 {
  LineBreakStrictness_option strictness;
  LineBreakWordOption_option word_option;
  OptionBool ja_zh;
} LineBreakOptionsV1;

typedef struct LineBreakOptionsV1_option {union { LineBreakOptionsV1 ok; }; bool is_ok; } LineBreakOptionsV1_option;



#endif // LineBreakOptionsV1_D_H
