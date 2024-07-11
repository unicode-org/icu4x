// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "PluralRules.hpp"
#include "Logger.hpp"

#include <iostream>

const std::string_view path = "../../provider/source/data/debug/";

int main() {
    Logger::init_simple_logger();
    std::unique_ptr<Locale> locale = Locale::create_from_string("ar").ok().value();
    std::cout << "Running test for locale " << locale->to_string() << std::endl;
    std::unique_ptr<DataProvider> dp = DataProvider::create_fs(path).ok().value();
    std::unique_ptr<PluralRules> pr = PluralRules::create_cardinal(*dp.get(), *locale.get()).ok().value();

    std::unique_ptr<PluralOperands> op = PluralOperands::create_from_string("3").ok().value();
    PluralCategory cat = pr->category_for(*op.get());

    std::cout << "Category is " << static_cast<int32_t>(cat)
                                << " (should be " << static_cast<int32_t>(PluralCategory::Value::Few) << ")"
                                << std::endl;
    if (cat != PluralCategory::Value::Few) {
        return 1;
    }

    op = PluralOperands::create_from_string("1011.0").ok().value();
    cat = pr->category_for(*op.get());
    std::cout << "Category is " << static_cast<int32_t>(cat)
                                << " (should be " << static_cast<int32_t>(PluralCategory::Value::Many) << ")"
                                << std::endl;
    if (cat != PluralCategory::Value::Many) {
        return 1;
    }
    return 0;
}
