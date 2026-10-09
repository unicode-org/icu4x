// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/PluralRules.hpp>
#include <icu4x/Decimal.hpp>
#include <icu4x/Logger.hpp>

#include <iostream>

using namespace icu4x;

const std::string_view path = "../../provider/source/data/debug/";

int main() {
    Logger::init_simple_logger();
    std::unique_ptr<Locale> locale = Locale::from_string("ar").ok().value();
    std::cout << "Running test for locale " << locale->to_string() << std::endl;
    std::unique_ptr<PluralRules> pr = PluralRules::create_cardinal(*locale.get()).ok().value();

    PluralCategory cat = pr->category_for(*PluralOperands::from(3).get());

    std::cout << "Category is " << static_cast<int32_t>(cat)
                                << " (should be " << static_cast<int32_t>(PluralCategory::Value::Few) << ")"
                                << std::endl;
    if (cat != PluralCategory::Value::Few) {
        return 1;
    }

    cat = pr->category_for(*PluralOperands::from_string("1011.0").ok()->get());
    std::cout << "Category is " << static_cast<int32_t>(cat)
                                << " (should be " << static_cast<int32_t>(PluralCategory::Value::Many) << ")"
                                << std::endl;
    if (cat != PluralCategory::Value::Many) {
        return 1;
    }

    // Test French compact plural category: 1,000,000 in compact notation has exponent 6 -> "many"
    std::unique_ptr<Locale> locale_fr = Locale::from_string("fr").ok().value();
    std::unique_ptr<PluralRules> pr_fr = PluralRules::create_cardinal(*locale_fr.get()).ok().value();

    std::unique_ptr<Decimal> dec_sig = Decimal::from_double_with_round_trip_precision(1.0).ok().value();
    std::unique_ptr<PluralOperands> operands_exp =
        PluralOperands::from_significand_and_exponent(*dec_sig.get(), 6);
    PluralCategory cat_fr = pr_fr->category_for(*operands_exp.get());
    std::cout << "French category for significand 1.0, exponent 6 is " << static_cast<int32_t>(cat_fr)
              << " (should be " << static_cast<int32_t>(PluralCategory::Value::Many) << ")"
              << std::endl;
    if (cat_fr != PluralCategory::Value::Many) {
        return 1;
    }

    return 0;
}

