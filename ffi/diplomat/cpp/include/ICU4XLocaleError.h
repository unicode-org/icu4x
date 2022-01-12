#ifndef ICU4XLocaleError_H
#define ICU4XLocaleError_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XLocaleError {
  ICU4XLocaleError_Undefined = 0,
  ICU4XLocaleError_Error = 1,
} ICU4XLocaleError;

void ICU4XLocaleError_destroy(ICU4XLocaleError* self);

#ifdef __cplusplus
}
#endif
#endif
