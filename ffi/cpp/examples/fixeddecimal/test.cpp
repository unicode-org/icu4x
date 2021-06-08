// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/decimal.hpp"

#include <iostream>

const std::string_view path = "../../../../provider/testdata/data/json/";
using namespace icu4x;

int main() {
    Locale locale("bn");
    std::cout << "Running test for locale " << locale.ToString().value() << std::endl;
    DataProvider dp = DataProvider::FsDataProvider(path).value();

    FixedDecimalFormatOptions opts = {GroupingStrategy::Auto, SignDisplay::Auto};
    FixedDecimalFormat fdf = FixedDecimalFormat::Create(locale, dp, opts).value();

    FixedDecimal decimal(1000007);
    std::string out = fdf.Format(decimal).value();
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "১০,০০,০০৭") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal.MultiplyPow10(2);
    decimal.Negate();
    out = fdf.Format(decimal).value();
    std::cout << "Value x100 and negated is " << out << std::endl;
    if (out != "-১০,০০,০০,৭০০") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }
    return 0;
}
