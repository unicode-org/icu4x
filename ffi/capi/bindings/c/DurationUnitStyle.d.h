#ifndef DurationUnitStyle_D_H
#define DurationUnitStyle_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DurationUnitStyle {
  DurationUnitStyle_Default = 0,
  DurationUnitStyle_Narrow = 1,
  DurationUnitStyle_Short = 2,
  DurationUnitStyle_Long = 3,
  DurationUnitStyle_TwoDigit = 4,
  DurationUnitStyle_Numeric = 5,
  DurationUnitStyle_Fractional = 6,
} DurationUnitStyle;

typedef struct DurationUnitStyle_option {union { DurationUnitStyle ok; }; bool is_ok; } DurationUnitStyle_option;



#endif // DurationUnitStyle_D_H
