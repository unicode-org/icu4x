#ifndef ICU4XLineBreakRule_type_H
#define ICU4XLineBreakRule_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef enum ICU4XLineBreakRule {
  ICU4XLineBreakRule_Loose = 0,
  ICU4XLineBreakRule_Normal = 1,
  ICU4XLineBreakRule_Strict = 2,
  ICU4XLineBreakRule_Anywhere = 3,
} ICU4XLineBreakRule;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XLineBreakRule_type_H
