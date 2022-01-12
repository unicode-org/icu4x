// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XFixedDecimalFormat.hpp"
#include "../../include/ICU4XDataStruct.hpp"

#include <iostream>
#include <array>

const std::string_view path = "../../../../../provider/testdata/data/json/";

int main() {
    ICU4XLocale locale = ICU4XLocale::create("bn").value();
    std::cout << "Running test for locale " << locale.tostring().ok().value() << std::endl;
    ICU4XDataProvider dp = ICU4XDataProvider::create_fs(path).provider.value();
    ICU4XFixedDecimalFormatOptions opts = {ICU4XFixedDecimalGroupingStrategy::Auto, ICU4XFixedDecimalSignDisplay::Auto};
    ICU4XFixedDecimalFormat fdf = ICU4XFixedDecimalFormat::try_new(locale, dp, opts).ok().value();

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

    decimal = ICU4XFixedDecimal::create_from_f64_with_max_precision(100.01).value();
    out = fdf.format(decimal).ok().value();
    std::cout << "Formatted float value is " << out << std::endl;
    if (out != "১০০.০১") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal = ICU4XFixedDecimal::create_from_f64_with_lower_magnitude(100.0006, -2, ICU4XFixedDecimalRoundingMode::HalfExpand).value();
    out = fdf.format(decimal).ok().value();
    std::cout << "Formatted float value from precision 2 is " << out << std::endl;
    if (out != "১০০.০০") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal = ICU4XFixedDecimal::create_from_f64_with_significant_digits(100.0006, 5, ICU4XFixedDecimalRoundingMode::HalfExpand).value();
    out = fdf.format(decimal).ok().value();
    std::cout << "Formatted float value with 5 digits is " << out << std::endl;
    if (out != "১০০.০০") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::array<char32_t, 10> digits = {U'a', U'b', U'c', U'd', U'e', U'f', U'g', U'h', U'i', U'j'};

    auto data = ICU4XDataStruct::create_decimal_symbols("+", "", "-", "", "/", "_", 4, 2, 4, digits).ok().value();

    fdf = ICU4XFixedDecimalFormat::try_new_from_decimal_symbols_v1(data, opts).ok().value();

    decimal = ICU4XFixedDecimal::create_from_f64_with_max_precision(123456.8901).value();
    out = fdf.format(decimal).ok().value();
    std::cout << "Formatted float value for custom numeric system is " << out << std::endl;
    if (out != "bcdefg/ijab") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }
    decimal = ICU4XFixedDecimal::create_from_f64_with_max_precision(123451234567.8901).value();
    out = fdf.format(decimal).ok().value();
    std::cout << "Formatted float value for custom numeric system is " << out << std::endl;
    if (out != "bc_de_fb_cd_efgh/ijab") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    return 0;
}
