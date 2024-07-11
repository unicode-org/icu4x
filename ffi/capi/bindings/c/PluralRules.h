#ifndef PluralRules_H
#define PluralRules_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "PluralCategories.d.h"
#include "PluralCategory.d.h"
#include "PluralOperands.d.h"

#include "PluralRules.d.h"






typedef struct ICU4XPluralRules_create_cardinal_result {union {PluralRules* ok; DataError err;}; bool is_ok;} ICU4XPluralRules_create_cardinal_result;
ICU4XPluralRules_create_cardinal_result ICU4XPluralRules_create_cardinal(const DataProvider* provider, const Locale* locale);

typedef struct ICU4XPluralRules_create_ordinal_result {union {PluralRules* ok; DataError err;}; bool is_ok;} ICU4XPluralRules_create_ordinal_result;
ICU4XPluralRules_create_ordinal_result ICU4XPluralRules_create_ordinal(const DataProvider* provider, const Locale* locale);

PluralCategory ICU4XPluralRules_category_for(const PluralRules* self, const PluralOperands* op);

PluralCategories ICU4XPluralRules_categories(const PluralRules* self);


void ICU4XPluralRules_destroy(PluralRules* self);





#endif // PluralRules_H
