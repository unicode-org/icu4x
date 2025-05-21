// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "DataProvider.h"
#include "Locale.h"
#include "PluralRules.h"
#include "PluralOperands.h"
#include "Logger.h"
#include <string.h>
#include <stdio.h>

int main() {
    icu4x_Logger_init_simple_logger_mv1();
    struct DiplomatStringView ar_str = {
        "ar",
        2
    };
    icu4x_Locale_from_string_mv1_result locale_result = icu4x_Locale_from_string_mv1(ar_str);
    if (!locale_result.is_ok) {
        return 1;
    }
    Locale* locale = locale_result.ok;
    icu4x_PluralRules_create_cardinal_mv1_result plural_result = icu4x_PluralRules_create_cardinal_mv1(locale);
    if (!plural_result.is_ok) {
        printf("Failed to create PluralRules\n");
        return 1;
    }
    PluralRules* rules = plural_result.ok;

    PluralCategories categories = icu4x_PluralRules_categories_mv1(rules);
    printf("Plural Category zero  (should be true): %s\n", categories.zero  ? "true" : "false");
    printf("Plural Category one   (should be true): %s\n", categories.one   ? "true" : "false");
    printf("Plural Category two   (should be true): %s\n", categories.two   ? "true" : "false");
    printf("Plural Category few   (should be true): %s\n", categories.few   ? "true" : "false");
    printf("Plural Category many  (should be true): %s\n", categories.many  ? "true" : "false");
    printf("Plural Category other (should be true): %s\n", categories.other ? "true" : "false");

    struct DiplomatStringView int_str = {
        "3",
        1
    };
    icu4x_PluralOperands_from_string_mv1_result op1_result = icu4x_PluralOperands_from_string_mv1(int_str);

    if (!op1_result.is_ok) {
        printf("Failed to create PluralOperands from string\n");
        return 1;
    }

    PluralCategory cat1 = icu4x_PluralRules_category_for_mv1(rules, op1_result.ok);

    printf("Plural Category %d (should be %d)\n", (int)cat1, (int)PluralCategory_Few);

    struct DiplomatStringView decimal_str = {
        "1011.0",
        6
    };
    icu4x_PluralOperands_from_string_mv1_result op2_result = icu4x_PluralOperands_from_string_mv1(decimal_str);

    if (!op2_result.is_ok) {
        printf("Failed to create PluralOperands from string\n");
        return 1;
    }

    PluralCategory cat2 = icu4x_PluralRules_category_for_mv1(rules, op2_result.ok);

    printf("Plural Category %d (should be %d)\n", (int)cat2, (int)PluralCategory_Many);

    icu4x_PluralRules_destroy_mv1(rules);
    icu4x_Locale_destroy_mv1(locale);

    if (!categories.zero)  { return 1; }
    if (!categories.one)   { return 1; }
    if (!categories.two)   { return 1; }
    if (!categories.few)   { return 1; }
    if (!categories.many)  { return 1; }
    if (!categories.other) { return 1; }

    if (cat1 != PluralCategory_Few)  { return 1; }
    if (cat2 != PluralCategory_Many) { return 1; }

    return 0;
}
