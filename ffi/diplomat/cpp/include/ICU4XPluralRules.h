#ifndef ICU4XPluralRules_H
#define ICU4XPluralRules_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XPluralRules ICU4XPluralRules;
#include "ICU4XLocale.h"
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XPluralRules_ICU4XError.h"
#include "ICU4XPluralOperands.h"
#include "ICU4XPluralCategory.h"
#include "ICU4XPluralCategories.h"

diplomat_result_box_ICU4XPluralRules_ICU4XError ICU4XPluralRules_try_new_cardinal(const ICU4XLocale* locale, const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XPluralRules_ICU4XError ICU4XPluralRules_try_new_ordinal(const ICU4XLocale* locale, const ICU4XDataProvider* provider);

ICU4XPluralCategory ICU4XPluralRules_select(const ICU4XPluralRules* self, ICU4XPluralOperands op);

ICU4XPluralCategories ICU4XPluralRules_categories(const ICU4XPluralRules* self);
void ICU4XPluralRules_destroy(ICU4XPluralRules* self);

#ifdef __cplusplus
}
#endif
#endif
