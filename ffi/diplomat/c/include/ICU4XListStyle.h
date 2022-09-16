#ifndef ICU4XListStyle_H
#define ICU4XListStyle_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XListStyle {
  ICU4XListStyle_Wide = 0,
  ICU4XListStyle_Short = 1,
  ICU4XListStyle_Narrow = 2,
} ICU4XListStyle;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XListStyle_destroy(ICU4XListStyle* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
