#ifndef Duration_D_H
#define Duration_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DurationSign.d.h"




typedef struct Duration {
  DurationSign sign;
  uint64_t years;
  uint64_t months;
  uint64_t weeks;
  uint64_t days;
  uint64_t hours;
  uint64_t minutes;
  uint64_t seconds;
  uint64_t milliseconds;
  uint64_t microseconds;
  uint64_t nanoseconds;
} Duration;

typedef struct Duration_option {union { Duration ok; }; bool is_ok; } Duration_option;



#endif // Duration_D_H
