#ifndef ICU4XHourCyclePreference_H
#define ICU4XHourCyclePreference_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XHourCyclePreference {
  ICU4XHourCyclePreference_H24 = 0,
  ICU4XHourCyclePreference_H23 = 1,
  ICU4XHourCyclePreference_H12 = 2,
  ICU4XHourCyclePreference_H11 = 3,
  ICU4XHourCyclePreference_None = 4,
} ICU4XHourCyclePreference;

void ICU4XHourCyclePreference_destroy(ICU4XHourCyclePreference* self);

#ifdef __cplusplus
}
#endif
#endif
