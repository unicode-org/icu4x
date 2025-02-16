#ifndef TimePrecision_D_H
#define TimePrecision_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum TimePrecision {
  TimePrecision_Hour = 0,
  TimePrecision_Minute = 1,
  TimePrecision_MinuteOptional = 2,
  TimePrecision_Second = 3,
  TimePrecision_SecondF1 = 4,
  TimePrecision_SecondF2 = 5,
  TimePrecision_SecondF3 = 6,
  TimePrecision_SecondF4 = 7,
  TimePrecision_SecondF5 = 8,
  TimePrecision_SecondF6 = 9,
  TimePrecision_SecondF7 = 10,
  TimePrecision_SecondF8 = 11,
  TimePrecision_SecondF9 = 12,
} TimePrecision;

typedef struct TimePrecision_option {union { TimePrecision ok; }; bool is_ok; } TimePrecision_option;



#endif // TimePrecision_D_H
