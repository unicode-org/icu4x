// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XFixedDecimalFormat.hpp"

#include <iostream>

const std::string_view path = "../../../../../provider/testdata/data/json/";

int main() {
    ICU4XLocale locale = ICU4XLocale::create("bn").value();
    std::cout << "Running test for locale " << locale.tostring().ok().value() << std::endl;
    ICU4XDataProvider dp = ICU4XDataProvider::create_fs(path).provider.value();
    ICU4XFixedDecimalFormatOptions opts = {ICU4XFixedDecimalGroupingStrategy::Auto, ICU4XFixedDecimalSignDisplay::Auto};
    ICU4XFixedDecimalFormat fdf = ICU4XFixedDecimalFormat::try_new(locale, dp, opts).fdf.value();

    ICU4XFixedDecimal decimal = ICU4XFixedDecimal::create(1000007);
    std::string out = fdf.format(decimal).ok().value();
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "১০,০০,০০৭") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::string out2;
    fdf.format_to_writeable(decimal, out2);
    std::cout << "Formatted writeable value is " << out2 << std::endl;
    if (out2 != "১০,০০,০০৭") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal.multiply_pow10(2);
    decimal.negate();
    out = fdf.format(decimal).ok().value();
    std::cout << "Value x100 and negated is " << out << std::endl;
    if (out != "-১০,০০,০০,৭০০") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    auto result = ICU4XFixedDecimal::create_from_f32(1234.5678);
    if (!result.success) {
        std::cout << "Constructing ICU4XFixedDecimal from float failed" << std::endl;
        return 1;
    }
    // std::move necessary since ICU4XFixedDecimal doesn't have copy constructors
    ICU4XFixedDecimal decimal2 = std::move(result.fd.value());
    out = fdf.format(decimal2).ok().value();
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "১,২৩৪.৫৬৭৭") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }
    return 0;
}
