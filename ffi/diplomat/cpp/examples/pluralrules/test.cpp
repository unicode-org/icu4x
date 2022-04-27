// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XPluralRules.hpp"

#include <iostream>

int main() {
    ICU4XLocale locale = ICU4XLocale::create("ar").value();
    std::cout << "Running test for locale " << locale.tostring().ok().value() << std::endl;
    ICU4XDataProvider dp = ICU4XDataProvider::create_test().provider.value();
    ICU4XPluralRules pr = ICU4XPluralRules::try_new_cardinal(locale, dp).rules.value();

    ICU4XPluralOperands op = { .i = 3 };
    ICU4XPluralCategory cat = pr.select(op);

    std::cout << "Category is " << static_cast<int32_t>(cat)
                                << " (should be " << static_cast<int32_t>(ICU4XPluralCategory::Few) << ")"
                                << std::endl;
    if (cat != ICU4XPluralCategory::Few) {
        return 1;
    }
    return 0;
}
