#ifndef ICU4XPluralCategories_type_H
#define ICU4XPluralCategories_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct ICU4XPluralCategories {
    bool zero;
    bool one;
    bool two;
    bool few;
    bool many;
    bool other;
} ICU4XPluralCategories;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XPluralCategories_type_H
