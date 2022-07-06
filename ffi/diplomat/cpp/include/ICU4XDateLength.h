#ifndef ICU4XDateLength_H
#define ICU4XDateLength_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XDateLength {
  ICU4XDateLength_Full = 0,
  ICU4XDateLength_Long = 1,
  ICU4XDateLength_Medium = 2,
  ICU4XDateLength_Short = 3,
} ICU4XDateLength;

void ICU4XDateLength_destroy(ICU4XDateLength* self);

#ifdef __cplusplus
}
#endif
#endif
