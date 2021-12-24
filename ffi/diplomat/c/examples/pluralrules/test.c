// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XPluralRules.h"
#include <string.h>
#include <stdio.h>

const char* path = "../../../../../provider/testdata/data/json/";
int main() {
    ICU4XLocale* locale = ICU4XLocale_create("ar", 2);
    ICU4XCreateDataProviderResult result = ICU4XDataProvider_create_fs(path, strlen(path));
    if (!result.success) {
        printf("Failed to create FsDataProvider\n");
        return 1;
    }
    ICU4XDataProvider* provider = result.provider;
    ICU4XCreatePluralRulesResult plural_result = ICU4XPluralRules_try_new(locale, provider, ICU4XPluralRuleType_Cardinal);
    if (!plural_result.success) {
        printf("Failed to create PluralRules\n");
        return 1;
    }
    ICU4XPluralRules* rules = plural_result.rules;

    ICU4XPluralCategories categories = ICU4XPluralRules_categories(rules);
    printf("Plural Category zero  (should be true): %s\n", categories.zero  ? "true" : "false");
    printf("Plural Category one   (should be true): %s\n", categories.one   ? "true" : "false");
    printf("Plural Category two   (should be true): %s\n", categories.two   ? "true" : "false");
    printf("Plural Category few   (should be true): %s\n", categories.few   ? "true" : "false");
    printf("Plural Category many  (should be true): %s\n", categories.many  ? "true" : "false");
    printf("Plural Category other (should be true): %s\n", categories.other ? "true" : "false");

    ICU4XPluralOperands op1 = { .i = 3 };

    ICU4XPluralCategory cat1 = ICU4XPluralRules_select(rules, op1);

    printf("Plural Category %d (should be %d)\n", (int)cat1, (int)ICU4XPluralCategory_Few);

    ICU4XCreatePluralOperandsResult op_result = ICU4XPluralOperands_create("1011.0", 6);

    if (!op_result.success) {
        printf("Failed to create PluralOperands from string\n");
        return 1;
    }

    ICU4XPluralCategory cat2 = ICU4XPluralRules_select(rules, op_result.operands);

    printf("Plural Category %d (should be %d)\n", (int)cat2, (int)ICU4XPluralCategory_Many);

    ICU4XPluralRules_destroy(rules);
    ICU4XDataProvider_destroy(provider);
    ICU4XLocale_destroy(locale);

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
