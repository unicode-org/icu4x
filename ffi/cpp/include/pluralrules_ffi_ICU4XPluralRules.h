#ifndef pluralrules_ffi_ICU4XPluralRules_H
#define pluralrules_ffi_ICU4XPluralRules_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XPluralRules ICU4XPluralRules;
#include "locale_ffi_ICU4XLocale.h"
#include "provider_ffi_ICU4XDataProvider.h"
#include "pluralrules_ffi_ICU4XPluralRuleType.h"
#include "pluralrules_ffi_ICU4XCreatePluralRulesResult.h"
#include "pluralrules_ffi_ICU4XPluralOperands.h"
#include "pluralrules_ffi_ICU4XPluralCategory.h"
#include "pluralrules_ffi_ICU4XPluralCategories.h"

ICU4XCreatePluralRulesResult ICU4XPluralRules_create(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XPluralRuleType ty);

ICU4XPluralCategory ICU4XPluralRules_select(const ICU4XPluralRules* self, const ICU4XPluralOperands* op);

ICU4XPluralCategories ICU4XPluralRules_categories(const ICU4XPluralRules* self);
void ICU4XPluralRules_destroy(ICU4XPluralRules* self);

#ifdef __cplusplus
}
#endif
#endif
