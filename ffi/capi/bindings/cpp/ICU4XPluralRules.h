#ifndef ICU4XPluralRules_H
#define ICU4XPluralRules_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "ICU4XPluralCategories.d.h"
#include "ICU4XPluralCategories.h"
#include "ICU4XPluralCategory.d.h"
#include "ICU4XPluralCategory.h"
#include "ICU4XPluralOperands.d.h"
#include "ICU4XPluralOperands.h"
#include "diplomat_result_box_ICU4XPluralRules_ICU4XDataError.d.h"

#include "ICU4XPluralRules.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XPluralRules_ICU4XDataError ICU4XPluralRules_create_cardinal(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_box_ICU4XPluralRules_ICU4XDataError ICU4XPluralRules_create_ordinal(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

ICU4XPluralCategory ICU4XPluralRules_category_for(const ICU4XPluralRules* self, const ICU4XPluralOperands* op);

ICU4XPluralCategories ICU4XPluralRules_categories(const ICU4XPluralRules* self);

void ICU4XPluralRules_destroy(ICU4XPluralRules* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XPluralRules_H
