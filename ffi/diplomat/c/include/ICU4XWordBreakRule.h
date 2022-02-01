#ifndef ICU4XWordBreakRule_H
#define ICU4XWordBreakRule_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XWordBreakRule {
  ICU4XWordBreakRule_Normal = 0,
  ICU4XWordBreakRule_BreakAll = 1,
  ICU4XWordBreakRule_KeepAll = 2,
} ICU4XWordBreakRule;

void ICU4XWordBreakRule_destroy(ICU4XWordBreakRule* self);

#ifdef __cplusplus
}
#endif
#endif
