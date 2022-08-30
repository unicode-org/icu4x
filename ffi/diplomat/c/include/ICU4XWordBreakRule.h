#ifndef ICU4XWordBreakRule_H
#define ICU4XWordBreakRule_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XWordBreakRule {
  ICU4XWordBreakRule_Normal = 0,
  ICU4XWordBreakRule_BreakAll = 1,
  ICU4XWordBreakRule_KeepAll = 2,
} ICU4XWordBreakRule;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XWordBreakRule_destroy(ICU4XWordBreakRule* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
