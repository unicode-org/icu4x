#ifndef ICU4XPluralRulesWithRanges_H
#define ICU4XPluralRulesWithRanges_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XPluralRulesWithRanges ICU4XPluralRulesWithRanges;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XPluralRulesWithRanges_ICU4XError.h"
#include "ICU4XPluralOperands.h"
#include "ICU4XPluralCategory.h"
#include "ICU4XPluralCategories.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XPluralRulesWithRanges_ICU4XError ICU4XPluralRulesWithRanges_create_cardinal(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_box_ICU4XPluralRulesWithRanges_ICU4XError ICU4XPluralRulesWithRanges_create_ordinal(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

ICU4XPluralCategory ICU4XPluralRulesWithRanges_category_for(const ICU4XPluralRulesWithRanges* self, const ICU4XPluralOperands* op);

ICU4XPluralCategories ICU4XPluralRulesWithRanges_categories(const ICU4XPluralRulesWithRanges* self);

ICU4XPluralCategory ICU4XPluralRulesWithRanges_category_for_range(const ICU4XPluralRulesWithRanges* self, const ICU4XPluralOperands* start, const ICU4XPluralOperands* end);

ICU4XPluralCategory ICU4XPluralRulesWithRanges_resolve_range(const ICU4XPluralRulesWithRanges* self, ICU4XPluralCategory start, ICU4XPluralCategory end);
void ICU4XPluralRulesWithRanges_destroy(ICU4XPluralRulesWithRanges* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
