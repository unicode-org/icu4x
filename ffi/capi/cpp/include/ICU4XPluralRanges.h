#ifndef ICU4XPluralRanges_H
#define ICU4XPluralRanges_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XPluralRanges ICU4XPluralRanges;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XPluralRanges_ICU4XError.h"
#include "ICU4XPluralCategory.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XPluralRanges_ICU4XError ICU4XPluralRanges_create(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

ICU4XPluralCategory ICU4XPluralRanges_category_for_range(const ICU4XPluralRanges* self, ICU4XPluralCategory start, ICU4XPluralCategory end);
void ICU4XPluralRanges_destroy(ICU4XPluralRanges* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
