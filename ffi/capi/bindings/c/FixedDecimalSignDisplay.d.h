#ifndef FixedDecimalSignDisplay_D_H
#define FixedDecimalSignDisplay_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum FixedDecimalSignDisplay {
  FixedDecimalSignDisplay_Auto = 0,
  FixedDecimalSignDisplay_Never = 1,
  FixedDecimalSignDisplay_Always = 2,
  FixedDecimalSignDisplay_ExceptZero = 3,
  FixedDecimalSignDisplay_Negative = 4,
} FixedDecimalSignDisplay;





#endif // FixedDecimalSignDisplay_D_H
