// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/DecimalFormatter.hpp>
#include <icu4x/Logger.hpp>

#include <iostream>
#include <array>

using namespace icu4x;

int main() {
    // For basic logging
    Logger::init_simple_logger();

    // Create a locale object representing Bangla
    std::unique_ptr<Locale> locale = Locale::from_string("bn").ok().value();

    std::cout << "Running test for locale " << locale->to_string() << std::endl;

    // Create a formatter object with the appropriate settings
    std::unique_ptr<DecimalFormatter> formatter = DecimalFormatter::create_with_grouping_strategy(
        *locale.get(), DecimalGroupingStrategy::Auto).ok().value();

    // Create a decimal representing the number 1,000,007
    std::unique_ptr<Decimal> decimal = Decimal::from(1000007);
    
    // Format it to a string
    std::string out = formatter->format(*decimal.get());
    
    // Report formatted value
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "১০,০০,০০৭") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal->multiply_pow10(2);
    decimal->set_sign(DecimalSign::Negative);
    out = formatter->format(*decimal.get());
    std::cout << "Value x100 and negated is " << out << std::endl;
    if (out != "-১০,০০,০০,৭০০") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal = Decimal::from_double_with_round_trip_precision(100.01).ok().value();
    out = formatter->format(*decimal.get());
    std::cout << "Formatted float value is " << out << std::endl;
    if (out != "১০০.০১") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal->pad_end(-4);
    out = formatter->format(*decimal.get());
    std::cout << "Formatted left-padded float value is " << out << std::endl;
    if (out != "১০০.০১০০") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal->pad_start(4);
    out = formatter->format(*decimal.get());
    std::cout << "Formatted right-padded float value is " << out << std::endl;
    if (out != "০,১০০.০১০০") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal->set_max_position(3);
    out = formatter->format(*decimal.get());
    std::cout << "Formatted truncated float value is " << out << std::endl;
    if (out != "১০০.০১০০") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal = Decimal::from_double_with_lower_magnitude(100.0006, -2).ok().value();
    out = formatter->format(*decimal.get());
    std::cout << "Formatted float value from precision 2 is " << out << std::endl;
    if (out != "১০০.০০") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    decimal = Decimal::from_double_with_significant_digits(100.0006, 5).ok().value();
    out = formatter->format(*decimal.get());
    std::cout << "Formatted float value with 5 digits is " << out << std::endl;
    if (out != "১০০.০০") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::array<char32_t, 10> digits = {U'a', U'b', U'c', U'd', U'e', U'f', U'g', U'h', U'i', U'j'};

    formatter = DecimalFormatter::create_with_manual_data("+", "", "-", "", "/", "_", 4, 2, 4, digits, DecimalGroupingStrategy::Auto).ok().value();

    decimal = Decimal::from_double_with_round_trip_precision(123456.8901).ok().value();
    out = formatter->format(*decimal.get());
    std::cout << "Formatted float value for custom numeric system is " << out << std::endl;
    if (out != "bcdefg/ijab") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }
    decimal = Decimal::from_double_with_round_trip_precision(123451234567.8901).ok().value();
    out = formatter->format(*decimal.get());
    std::cout << "Formatted float value for custom numeric system is " << out << std::endl;
    if (out != "bc_de_fb_cd_efgh/ijab") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    locale = Locale::from_string("th-u-nu-thai").ok().value();
    std::cout << "Running test for locale " << locale->to_string() << std::endl;
    formatter = DecimalFormatter::create_with_grouping_strategy(
        *locale.get(), DecimalGroupingStrategy::Auto).ok().value();

    decimal = Decimal::from_double_with_round_trip_precision(123456.8901).ok().value();
    out = formatter->format(*decimal.get());
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "๑๒๓,๔๕๖.๘๙๐๑") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }
    return 0;
}
