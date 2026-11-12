// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/Duration.hpp>
#include <icu4x/DurationBaseStyle.hpp>
#include <icu4x/DurationFieldDisplay.hpp>
#include <icu4x/DurationFormatter.hpp>
#include <icu4x/DurationFormatterError.hpp>
#include <icu4x/DurationFormatterOptionsV1.hpp>
#include <icu4x/DurationSign.hpp>
#include <icu4x/DurationUnitStyle.hpp>
#include <icu4x/Locale.hpp>
#include <icu4x/Logger.hpp>

#include <iostream>
#include <memory>
#include <string>

using namespace icu4x;

int main() {
    Logger::init_simple_logger();
    std::unique_ptr<Locale> locale = Locale::from_string("en").ok().value();

    // 1. Default Short style
    {
        DurationFormatterOptionsV1 options = DurationFormatterOptionsV1();
        options.base = DurationBaseStyle::Short;
        std::unique_ptr<DurationFormatter> formatter =
            DurationFormatter::create_v1(*locale.get(), options).ok().value();

        Duration duration = {
            /* .sign = */ DurationSign::Positive,
            /* .years = */ 1,
            /* .months = */ 2,
            /* .weeks = */ 3,
            /* .days = */ 4,
            /* .hours = */ 5,
            /* .minutes = */ 6,
            /* .seconds = */ 7,
            /* .milliseconds = */ 0,
            /* .microseconds = */ 0,
            /* .nanoseconds = */ 0,
        };

        std::string out = formatter->format(duration);
        std::cout << "Formatted duration (short): " << out << std::endl;
        if (out != "1 yr, 2 mths, 3 wks, 4 days, 5 hr, 6 min, 7 sec") {
            std::cout << "Output does not match expected output" << std::endl;
            return 1;
        }
    }

    // 2. Digital style with negative sign, always-visible years, and fractional subseconds
    {
        DurationFormatterOptionsV1 options = DurationFormatterOptionsV1();
        options.base = DurationBaseStyle::Digital;
        options.year_visibility = DurationFieldDisplay::Always;

        std::unique_ptr<DurationFormatter> formatter =
            DurationFormatter::create_v1(*locale.get(), options).ok().value();

        Duration duration = {
            /* .sign = */ DurationSign::Negative,
            /* .years = */ 0,
            /* .months = */ 2,
            /* .weeks = */ 3,
            /* .days = */ 0,
            /* .hours = */ 12,
            /* .minutes = */ 1,
            /* .seconds = */ 5,
            /* .milliseconds = */ 130,
            /* .microseconds = */ 140,
            /* .nanoseconds = */ 0,
        };

        std::string out = formatter->format(duration);
        std::cout << "Formatted duration (digital): " << out << std::endl;
        if (out != "-0 yrs, 2 mths, 3 wks, 12:01:05.13014") {
            std::cout << "Output does not match expected output" << std::endl;
            return 1;
        }
    }

    // 3. Invalid options validation
    {
        DurationFormatterOptionsV1 options = DurationFormatterOptionsV1();
        options.base = DurationBaseStyle::Short;
        options.fractional_digits = 10;
        auto res = DurationFormatter::create_v1(*locale.get(), options);
        if (res.is_ok() || std::move(res).err().value() != DurationFormatterError::FractionalDigitsOutOfRange) {
            std::cout << "Expected FractionalDigitsOutOfRange error" << std::endl;
            return 1;
        }
    }

    return 0;
}
