#ifndef ICU4XLineBreakRule_H
#define ICU4XLineBreakRule_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XLineBreakRule {
  ICU4XLineBreakRule_Loose = 0,
  ICU4XLineBreakRule_Normal = 1,
  ICU4XLineBreakRule_Strict = 2,
  ICU4XLineBreakRule_Anywhere = 3,
} ICU4XLineBreakRule;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XLineBreakRule_destroy(ICU4XLineBreakRule* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
