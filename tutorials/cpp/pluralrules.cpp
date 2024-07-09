// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "ICU4XPluralRules.hpp"
#include "ICU4XLogger.hpp"

#include <iostream>

const std::string_view path = "../../provider/source/data/debug/";

int main() {
    ICU4XLogger::init_simple_logger();
    std::unique_ptr<ICU4XLocale> locale = ICU4XLocale::create_from_string("ar").ok().value();
    std::cout << "Running test for locale " << locale->to_string() << std::endl;
    std::unique_ptr<ICU4XDataProvider> dp = ICU4XDataProvider::create_fs(path).ok().value();
    std::unique_ptr<ICU4XPluralRules> pr = ICU4XPluralRules::create_cardinal(*dp.get(), *locale.get()).ok().value();

    std::unique_ptr<ICU4XPluralOperands> op = ICU4XPluralOperands::create_from_string("3").ok().value();
    ICU4XPluralCategory cat = pr->category_for(*op.get());

    std::cout << "Category is " << static_cast<int32_t>(cat)
                                << " (should be " << static_cast<int32_t>(ICU4XPluralCategory::Value::Few) << ")"
                                << std::endl;
    if (cat != ICU4XPluralCategory::Value::Few) {
        return 1;
    }

    op = ICU4XPluralOperands::create_from_string("1011.0").ok().value();
    cat = pr->category_for(*op.get());
    std::cout << "Category is " << static_cast<int32_t>(cat)
                                << " (should be " << static_cast<int32_t>(ICU4XPluralCategory::Value::Many) << ")"
                                << std::endl;
    if (cat != ICU4XPluralCategory::Value::Many) {
        return 1;
    }
    return 0;
}
