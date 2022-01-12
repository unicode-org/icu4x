#ifndef ICU4XCanonicalizationResult_H
#define ICU4XCanonicalizationResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XCanonicalizationResult {
  ICU4XCanonicalizationResult_Modified = 0,
  ICU4XCanonicalizationResult_Unmodified = 1,
} ICU4XCanonicalizationResult;

void ICU4XCanonicalizationResult_destroy(ICU4XCanonicalizationResult* self);

#ifdef __cplusplus
}
#endif
#endif
