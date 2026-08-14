// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/DateAddOptions.hpp>
#include <icu4x/DateDuration.hpp>
#include <icu4x/DateRangeFormatter.hpp>
#include <icu4x/DateRangeFormatterGregorian.hpp>
#include <icu4x/DateTimeRangeFormatter.hpp>
#include <icu4x/DateTimeRangeFormatterGregorian.hpp>
#include <icu4x/TimeRangeFormatter.hpp>
#include <icu4x/Logger.hpp>

#include <iostream>
#include <memory>
#include <optional>
#include <string>

using namespace icu4x;

int main() {
    Logger::init_simple_logger();
    std::unique_ptr<Locale> locale = Locale::from_string("en").ok().value();
    std::cout << "Running range formatter C++ test for locale " << locale->to_string() << std::endl;

    bool saw_unexpected_output = false;

    // Dates for range testing
    std::unique_ptr<IsoDate> start_date = IsoDate::create(2023, 12, 22).ok().value();
    DateDuration one_day = DateDuration::for_days(1);
    DateAddOptions add_options = { /* .overflow = */ std::nullopt };
    std::unique_ptr<IsoDate> end_next_day = start_date->try_add_with_options(one_day, add_options).ok().value();

    // Times for range testing
    std::unique_ptr<Time> start_time = Time::create(9, 0, 0, 0).ok().value();
    std::unique_ptr<Time> end_time = Time::create(17, 0, 0, 0).ok().value();

    std::string out;

    // 1. DateRangeFormatter (YMD medium)
    {
        std::unique_ptr<DateRangeFormatter> fmt = DateRangeFormatter::create_ymd(*locale.get(), DateTimeLength::Medium, std::nullopt, std::nullopt).ok().value();
        out = fmt->format_iso(*start_date.get(), *end_next_day.get());
        std::cout << "DateRangeFormatter YMD: " << out;
        if (out != "Dec 22 – 23, 2023") {
            std::cout << " (unexpected!)";
            saw_unexpected_output = true;
        }
        std::cout << std::endl;
    }

    // 2. DateRangeFormatterGregorian (YMD medium)
    {
        std::unique_ptr<DateRangeFormatterGregorian> fmt = DateRangeFormatterGregorian::create_ymd(*locale.get(), DateTimeLength::Medium, std::nullopt, std::nullopt).ok().value();
        out = fmt->format_iso(*start_date.get(), *end_next_day.get());
        std::cout << "DateRangeFormatterGregorian YMD: " << out;
        if (out != "Dec 22 – 23, 2023") {
            std::cout << " (unexpected!)";
            saw_unexpected_output = true;
        }
        std::cout << std::endl;
    }

    // 3. TimeRangeFormatter (T medium)
    {
        std::unique_ptr<TimeRangeFormatter> fmt = TimeRangeFormatter::create(*locale.get(), DateTimeLength::Medium, std::nullopt, std::nullopt).ok().value();
        out = fmt->format(*start_time.get(), *end_time.get());
        std::cout << "TimeRangeFormatter: " << out;
        if (out != "9:00 AM – 5:00 PM") {
            std::cout << " (unexpected!)";
            saw_unexpected_output = true;
        }
        std::cout << std::endl;
    }

    // 4. DateTimeRangeFormatter (YMDT medium)
    {
        std::unique_ptr<DateTimeRangeFormatter> fmt = DateTimeRangeFormatter::create_ymdt(*locale.get(), DateTimeLength::Medium, std::nullopt, std::nullopt, std::nullopt).ok().value();
        out = fmt->format_iso(*start_date.get(), *start_time.get(), *end_next_day.get(), *end_time.get());
        std::cout << "DateTimeRangeFormatter YMDT: " << out;
        if (out != "Dec 22, 2023, 9:00:00 AM – Dec 23, 2023, 5:00:00 PM") {
            std::cout << " (unexpected!)";
            saw_unexpected_output = true;
        }
        std::cout << std::endl;
    }

    if (saw_unexpected_output) {
        return 1;
    }
    return 0;
}
