#ifndef ICU4XPluralCategory_type_H
#define ICU4XPluralCategory_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

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
#endif // __cplusplus
#endif // ICU4XPluralCategory_type_H
