#ifndef CollatorNumeric_D_H
#define CollatorNumeric_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CollatorNumeric {
  CollatorNumeric_Auto = 0,
  CollatorNumeric_Off = 1,
  CollatorNumeric_On = 2,
} CollatorNumeric;

typedef struct CollatorNumeric_option {union { CollatorNumeric ok; }; bool is_ok; } CollatorNumeric_option;



#endif // CollatorNumeric_D_H
