#ifndef TimeLength_D_H
#define TimeLength_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum TimeLength {
  TimeLength_Full = 0,
  TimeLength_Long = 1,
  TimeLength_Medium = 2,
  TimeLength_Short = 3,
} TimeLength;

typedef struct TimeLength_option {union { TimeLength ok; }; bool is_ok; } TimeLength_option;



#endif // TimeLength_D_H
