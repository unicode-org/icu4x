#ifndef ICU4XCanonicalizationResult_H
#define ICU4XCanonicalizationResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XCanonicalizationResult {
  ICU4XCanonicalizationResult_Modified = 0,
  ICU4XCanonicalizationResult_Unmodified = 1,
} ICU4XCanonicalizationResult;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XCanonicalizationResult_destroy(ICU4XCanonicalizationResult* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
