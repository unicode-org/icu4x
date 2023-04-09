#ifndef ICU4XSegmenterRuleStatusType_H
#define ICU4XSegmenterRuleStatusType_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XSegmenterRuleStatusType {
  ICU4XSegmenterRuleStatusType_None = 0,
  ICU4XSegmenterRuleStatusType_Number = 1,
  ICU4XSegmenterRuleStatusType_Letter = 2,
} ICU4XSegmenterRuleStatusType;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XSegmenterRuleStatusType_destroy(ICU4XSegmenterRuleStatusType* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
