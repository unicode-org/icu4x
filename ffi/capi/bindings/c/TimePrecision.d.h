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
  TimePrecision_SecondS1 = 4,
  TimePrecision_SecondS2 = 5,
  TimePrecision_SecondS3 = 6,
  TimePrecision_SecondS4 = 7,
  TimePrecision_SecondS5 = 8,
  TimePrecision_SecondS6 = 9,
  TimePrecision_SecondS7 = 10,
  TimePrecision_SecondS8 = 11,
  TimePrecision_SecondS9 = 12,
} TimePrecision;

typedef struct TimePrecision_option {union { TimePrecision ok; }; bool is_ok; } TimePrecision_option;



#endif // TimePrecision_D_H
