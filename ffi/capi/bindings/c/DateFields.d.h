#ifndef DateFields_D_H
#define DateFields_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DateFields {
  DateFields_D = 0,
  DateFields_MD = 1,
  DateFields_YMD = 2,
  DateFields_DE = 3,
  DateFields_MDE = 4,
  DateFields_YMDE = 5,
  DateFields_E = 6,
  DateFields_M = 7,
  DateFields_YM = 8,
  DateFields_Y = 9,
} DateFields;

typedef struct DateFields_option {union { DateFields ok; }; bool is_ok; } DateFields_option;



#endif // DateFields_D_H
