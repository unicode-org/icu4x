// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/pluralrules.h"
#include <string.h>
#include <stdio.h>

const char* path = "../../../../provider/testdata/data/json/";
int main() {
    ICU4XLocale* locale = icu4x_locale_create("ar", 2);
    ICU4XCreateDataProviderResult result = icu4x_fs_data_provider_create(path, strlen(path));
    if (!result.success) {
        printf("Failed to create FsDataProvider\n");
        return 1;
    }
    ICU4XDataProvider provider = result.provider;
    ICU4XCreatePluralRulesResult plural_result = icu4x_plural_rules_create(locale, &provider, ICU4XPluralRuleType_Cardinal);
    if (!plural_result.success) {
        printf("Failed to create PluralRules\n");
        return 1;
    }
    ICU4XPluralRules* rules = plural_result.rules;

    ICU4XPluralOperands op = { .i = 3 };

    ICU4XPluralCategory cat = icu4x_plural_rules_select(rules, &op);

    printf("Plural Category %d (should be %d)\n", (int)cat, (int)ICU4XPluralCategory_Few);

    icu4x_plural_rules_destroy(rules);
    icu4x_data_provider_destroy(provider);
    icu4x_locale_destroy(locale);

    if (cat != ICU4XPluralCategory_Few) {
        return 1;
    }
    return 0;
}
