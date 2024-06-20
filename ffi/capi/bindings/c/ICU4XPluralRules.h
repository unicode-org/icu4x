#ifndef ICU4XPluralRules_H
#define ICU4XPluralRules_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XPluralCategories.d.h"
#include "ICU4XPluralCategory.d.h"
#include "ICU4XPluralOperands.d.h"

#include "ICU4XPluralRules.d.h"






struct ICU4XPluralRules_create_cardinal_result {union {ICU4XPluralRules* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XPluralRules_create_cardinal_result ICU4XPluralRules_create_cardinal(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

struct ICU4XPluralRules_create_ordinal_result {union {ICU4XPluralRules* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XPluralRules_create_ordinal_result ICU4XPluralRules_create_ordinal(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

ICU4XPluralCategory ICU4XPluralRules_category_for(const ICU4XPluralRules* self, const ICU4XPluralOperands* op);

ICU4XPluralCategories ICU4XPluralRules_categories(const ICU4XPluralRules* self);


void ICU4XPluralRules_destroy(ICU4XPluralRules* self);





#endif // ICU4XPluralRules_H
