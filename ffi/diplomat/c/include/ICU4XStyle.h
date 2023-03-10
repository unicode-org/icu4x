#ifndef ICU4XStyle_H
#define ICU4XStyle_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XStyle {
  ICU4XStyle_Auto = 0,
  ICU4XStyle_Narrow = 1,
  ICU4XStyle_Short = 2,
  ICU4XStyle_Long = 3,
  ICU4XStyle_Menu = 4,
} ICU4XStyle;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XStyle_destroy(ICU4XStyle* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
