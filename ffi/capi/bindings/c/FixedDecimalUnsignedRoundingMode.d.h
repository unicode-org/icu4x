#ifndef FixedDecimalUnsignedRoundingMode_D_H
#define FixedDecimalUnsignedRoundingMode_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum FixedDecimalUnsignedRoundingMode {
  FixedDecimalUnsignedRoundingMode_Expand = 0,
  FixedDecimalUnsignedRoundingMode_Trunc = 1,
  FixedDecimalUnsignedRoundingMode_HalfExpand = 2,
  FixedDecimalUnsignedRoundingMode_HalfTrunc = 3,
  FixedDecimalUnsignedRoundingMode_HalfEven = 4,
} FixedDecimalUnsignedRoundingMode;

typedef struct FixedDecimalUnsignedRoundingMode_option {union { FixedDecimalUnsignedRoundingMode ok; }; bool is_ok; } FixedDecimalUnsignedRoundingMode_option;



#endif // FixedDecimalUnsignedRoundingMode_D_H
