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
  LineBreakStrictness strictness;
  LineBreakWordOption word_option;
  bool ja_zh;
} LineBreakOptionsV1;





#endif // LineBreakOptionsV1_D_H
