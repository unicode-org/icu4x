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

    ICU4XPluralCategories categories = icu4x_plural_rules_categories(rules);
    printf("Plural Category zero  (should be true): %s\n", categories.zero  ? "true" : "false");
    printf("Plural Category one   (should be true): %s\n", categories.one   ? "true" : "false");
    printf("Plural Category two   (should be true): %s\n", categories.two   ? "true" : "false");
    printf("Plural Category few   (should be true): %s\n", categories.few   ? "true" : "false");
    printf("Plural Category many  (should be true): %s\n", categories.many  ? "true" : "false");
    printf("Plural Category other (should be true): %s\n", categories.other ? "true" : "false");

    ICU4XPluralOperands op1 = { .i = 3 };

    ICU4XPluralCategory cat1 = icu4x_plural_rules_select(rules, &op1);

    printf("Plural Category %d (should be %d)\n", (int)cat1, (int)ICU4XPluralCategory_Few);

    ICU4XCreatePluralOperandsResult op_result = icu4x_plural_operands_create("1011.0", 6);

    if (!op_result.success) {
        printf("Failed to create PluralOperands from string\n");
        return 1;
    }

    ICU4XPluralCategory cat2 = icu4x_plural_rules_select(rules, &op_result.operands);

    printf("Plural Category %d (should be %d)\n", (int)cat2, (int)ICU4XPluralCategory_Many);

    icu4x_plural_rules_destroy(rules);
    icu4x_data_provider_destroy(provider);
    icu4x_locale_destroy(locale);

    if (!categories.zero)  { return 1; }
    if (!categories.one)   { return 1; }
    if (!categories.two)   { return 1; }
    if (!categories.few)   { return 1; }
    if (!categories.many)  { return 1; }
    if (!categories.other) { return 1; }

    if (cat1 != ICU4XPluralCategory_Few)  { return 1; }
    if (cat2 != ICU4XPluralCategory_Many) { return 1; }

    return 0;
}
