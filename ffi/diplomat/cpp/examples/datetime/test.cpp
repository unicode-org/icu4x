// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XGregorianDateFormatter.hpp"
#include "../../include/ICU4XGregorianDateTimeFormatter.hpp"
#include "../../include/ICU4XTimeFormatter.hpp"
#include "../../include/ICU4XDataStruct.hpp"

#include <atomic>
#include <iostream>
#include <array>

int main() {
    ICU4XLocale locale = ICU4XLocale::create("es").value();
    std::cout << "Running test for locale " << locale.tostring().ok().value() << std::endl;
    ICU4XDataProvider dp = ICU4XDataProvider::create_test();

    ICU4XGregorianDateTime date = ICU4XGregorianDateTime::try_new(2022, 07, 11, 13, 06, 42).ok().value();

    ICU4XTimeFormatter tf = ICU4XTimeFormatter::try_new(dp, locale, ICU4XTimeLength::Short, ICU4XHourCyclePreference::None).ok().value();
    std::string out = tf.format_gregorian_datetime(date).ok().value();
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "13:06") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    ICU4XGregorianDateFormatter df = ICU4XGregorianDateFormatter::try_new(dp, locale, ICU4XDateLength::Full).ok().value();
    out = df.format_datetime(date).ok().value();
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "lunes, 11 de julio de 2022") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    ICU4XGregorianDateTimeFormatter dtf = ICU4XGregorianDateTimeFormatter::try_new(dp, locale, ICU4XDateLength::Medium, ICU4XTimeLength::Short, ICU4XHourCyclePreference::None).ok().value();
    out = dtf.format_datetime(date).ok().value();
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "11 jul 2022, 13:06") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    return 0;
}
