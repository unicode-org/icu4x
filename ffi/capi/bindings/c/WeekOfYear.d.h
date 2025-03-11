#ifndef WeekOfYear_D_H
#define WeekOfYear_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct WeekOfYear {
  uint8_t week_number;
  int32_t iso_year;
} WeekOfYear;

typedef struct WeekOfYear_option {union { WeekOfYear ok; }; bool is_ok; } WeekOfYear_option;



#endif // WeekOfYear_D_H
