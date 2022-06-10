#ifndef ICU4XError_H
#define ICU4XError_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XError {
  ICU4XError_WriteableError = 1,
  ICU4XError_LocaleUndefinedSubtagError = 2,
  ICU4XError_LocaleParserError = 3,
} ICU4XError;

void ICU4XError_destroy(ICU4XError* self);

#ifdef __cplusplus
}
#endif
#endif
