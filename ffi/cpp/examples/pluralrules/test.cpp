// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/pluralrules.hpp"

#include <iostream>

const std::string_view path = "../../../../provider/testdata/data/json/";
using namespace icu4x;

int main() {
    Locale locale("ar");
    std::cout << "Running test for locale " << locale.ToString().value() << std::endl;
    DataProvider dp = DataProvider::FsDataProvider(path).value();
    PluralRules pr = PluralRules::Create(locale, dp, PluralRuleType::Cardinal).value();

    PluralOperands op = { .i = 3 };
    PluralCategory cat = pr.Select(op);

    std::cout << "Category is " << static_cast<int32_t>(cat)
                                << " (should be " << static_cast<int32_t>(PluralCategory::Few) << ")"
                                << std::endl;
    if (cat != PluralCategory::Few) {
        return 1;
    }
    return 0;
}
