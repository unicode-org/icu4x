#ifndef ICU4XPluralRules_H
#define ICU4XPluralRules_H
#include <stdio.h>
#include <uchar.h>
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
#include "ICU4XPluralRuleType.h"
#include "ICU4XCreatePluralRulesResult.h"
#include "ICU4XPluralOperands.h"
#include "ICU4XPluralCategory.h"
#include "ICU4XPluralCategories.h"

ICU4XCreatePluralRulesResult ICU4XPluralRules_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XPluralRuleType ty);

ICU4XPluralCategory ICU4XPluralRules_select(const ICU4XPluralRules* self, ICU4XPluralOperands op);

ICU4XPluralCategories ICU4XPluralRules_categories(const ICU4XPluralRules* self);
void ICU4XPluralRules_destroy(ICU4XPluralRules* self);

#ifdef __cplusplus
}
#endif
#endif
