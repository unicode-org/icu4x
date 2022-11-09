#ifndef ICU4XPluralRules_H
#define ICU4XPluralRules_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XPluralRules_type.h"
#include "ICU4XDataProvider_type.h"
#include "ICU4XLocale_type.h"
#include "diplomat_result_box_ICU4XPluralRules_ICU4XError.h"
#include "ICU4XPluralOperands_type.h"
#include "ICU4XPluralCategory_type.h"
#include "ICU4XPluralCategories_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XPluralRules_ICU4XError ICU4XPluralRules_create_cardinal(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_box_ICU4XPluralRules_ICU4XError ICU4XPluralRules_create_ordinal(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

ICU4XPluralCategory ICU4XPluralRules_category_for(const ICU4XPluralRules* self, const ICU4XPluralOperands* op);

ICU4XPluralCategories ICU4XPluralRules_categories(const ICU4XPluralRules* self);
void ICU4XPluralRules_destroy(ICU4XPluralRules* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XPluralRules_H
