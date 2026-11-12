#ifndef DurationSign_D_H
#define DurationSign_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DurationSign {
  DurationSign_Positive = 0,
  DurationSign_Negative = 1,
} DurationSign;

typedef struct DurationSign_option {union { DurationSign ok; }; bool is_ok; } DurationSign_option;



#endif // DurationSign_D_H
