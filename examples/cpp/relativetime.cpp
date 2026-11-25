// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/Decimal.hpp>
#include <icu4x/DecimalSign.hpp>
#include <icu4x/Locale.hpp>
#include <icu4x/Logger.hpp>
#include <icu4x/RelativeTimeFormatter.hpp>
#include <icu4x/RelativeTimeFormatterOptionsV1.hpp>
#include <icu4x/RelativeTimeNumeric.hpp>

#include <iostream>
#include <memory>
#include <optional>
#include <string>

using namespace icu4x;

int main() {
    Logger::init_simple_logger();
    std::unique_ptr<Locale> locale = Locale::from_string("en").ok().value();
    RelativeTimeFormatterOptionsV1 options = {
        /* .numeric = */ RelativeTimeNumeric::Auto,
        /* .grouping_strategy = */ std::nullopt,
    };
    std::unique_ptr<RelativeTimeFormatter> fmt =
        RelativeTimeFormatter::create_long_day(*locale, options).ok().value();

    // 1. `Numeric::Auto` relative literal (-1 -> "yesterday")
    std::unique_ptr<Decimal> minus_one = Decimal::from(-1);
    if (fmt->format(*minus_one) != "yesterday") {
        std::cout << "Unexpected format(-1): " << fmt->format(*minus_one) << std::endl;
        return 1;
    }

    // 2. `Numeric::Auto` with -0 -> "0 days ago"
    std::unique_ptr<Decimal> neg_zero = Decimal::from(0);
    neg_zero->set_sign(DecimalSign::Negative);
    if (fmt->format(*neg_zero) != "0 days ago") {
        std::cout << "Unexpected format(-0): " << fmt->format(*neg_zero) << std::endl;
        return 1;
    }

    // 3. Grouped number (1234 -> "in 1,234 days")
    std::unique_ptr<Decimal> thousand = Decimal::from(1234);
    std::string out_grouped = fmt->format(*thousand);
    if (out_grouped != "in 1,234 days") {
        std::cout << "Unexpected format(1234): " << out_grouped << std::endl;
        return 1;
    }

    // 4. Bengali ("bn", narrow year)
    std::unique_ptr<Locale> bn = Locale::from_string("bn").ok().value();
    std::unique_ptr<RelativeTimeFormatter> bn_fmt =
        RelativeTimeFormatter::create_narrow_year(*bn, {}).ok().value();
    std::unique_ptr<Decimal> three = Decimal::from(3);
    if (bn_fmt->format(*three) != "৩ বছরে") {
        std::cout << "Unexpected Bengali output: " << bn_fmt->format(*three) << std::endl;
        return 1;
    }

    std::cout << "RelativeTimeFormatter C++ test passed: " << out_grouped << std::endl;
    return 0;
}
