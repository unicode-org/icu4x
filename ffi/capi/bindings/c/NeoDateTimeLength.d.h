#ifndef NeoDateTimeLength_D_H
#define NeoDateTimeLength_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum NeoDateTimeLength {
  NeoDateTimeLength_Long = 0,
  NeoDateTimeLength_Medium = 1,
  NeoDateTimeLength_Short = 2,
} NeoDateTimeLength;

typedef struct NeoDateTimeLength_option {union { NeoDateTimeLength ok; }; bool is_ok; } NeoDateTimeLength_option;



#endif // NeoDateTimeLength_D_H
