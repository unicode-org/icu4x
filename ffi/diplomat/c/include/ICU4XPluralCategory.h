#ifndef ICU4XPluralCategory_H
#define ICU4XPluralCategory_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XPluralCategory {
  ICU4XPluralCategory_Zero = 0,
  ICU4XPluralCategory_One = 1,
  ICU4XPluralCategory_Two = 2,
  ICU4XPluralCategory_Few = 3,
  ICU4XPluralCategory_Many = 4,
  ICU4XPluralCategory_Other = 5,
} ICU4XPluralCategory;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XPluralCategory_destroy(ICU4XPluralCategory* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
