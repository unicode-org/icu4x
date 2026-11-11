#ifndef DurationBaseStyle_D_H
#define DurationBaseStyle_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DurationBaseStyle {
  DurationBaseStyle_Narrow = 0,
  DurationBaseStyle_Short = 1,
  DurationBaseStyle_Long = 2,
  DurationBaseStyle_Digital = 3,
} DurationBaseStyle;

typedef struct DurationBaseStyle_option {union { DurationBaseStyle ok; }; bool is_ok; } DurationBaseStyle_option;



#endif // DurationBaseStyle_D_H
