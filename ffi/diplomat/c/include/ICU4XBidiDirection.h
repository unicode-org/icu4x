#ifndef ICU4XBidiDirection_H
#define ICU4XBidiDirection_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XBidiDirection {
  ICU4XBidiDirection_Ltr = 0,
  ICU4XBidiDirection_Rtl = 1,
  ICU4XBidiDirection_Mixed = 2,
} ICU4XBidiDirection;

void ICU4XBidiDirection_destroy(ICU4XBidiDirection* self);

#ifdef __cplusplus
}
#endif
#endif
