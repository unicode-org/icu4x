#ifndef RelativeTimeNumeric_D_H
#define RelativeTimeNumeric_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum RelativeTimeNumeric {
  RelativeTimeNumeric_Always = 0,
  RelativeTimeNumeric_Auto = 1,
} RelativeTimeNumeric;

typedef struct RelativeTimeNumeric_option {union { RelativeTimeNumeric ok; }; bool is_ok; } RelativeTimeNumeric_option;



#endif // RelativeTimeNumeric_D_H
