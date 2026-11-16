#ifndef DurationUnitStyle_D_H
#define DurationUnitStyle_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DurationUnitStyle {
  DurationUnitStyle_Narrow = 0,
  DurationUnitStyle_Short = 1,
  DurationUnitStyle_Long = 2,
  DurationUnitStyle_TwoDigit = 3,
  DurationUnitStyle_Numeric = 4,
  DurationUnitStyle_Fractional = 5,
} DurationUnitStyle;

typedef struct DurationUnitStyle_option {union { DurationUnitStyle ok; }; bool is_ok; } DurationUnitStyle_option;



#endif // DurationUnitStyle_D_H
