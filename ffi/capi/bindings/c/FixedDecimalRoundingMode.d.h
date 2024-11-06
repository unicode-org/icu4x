#ifndef FixedDecimalRoundingMode_D_H
#define FixedDecimalRoundingMode_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum FixedDecimalRoundingMode {
  FixedDecimalRoundingMode_Ceil = 0,
  FixedDecimalRoundingMode_Floor = 1,
  FixedDecimalRoundingMode_HalfCeil = 2,
  FixedDecimalRoundingMode_HalfFloor = 3,
} FixedDecimalRoundingMode;

typedef struct FixedDecimalRoundingMode_option {union { FixedDecimalRoundingMode ok; }; bool is_ok; } FixedDecimalRoundingMode_option;



#endif // FixedDecimalRoundingMode_D_H
