// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XGregorianDateFormatter.hpp"
#include "../../include/ICU4XGregorianDateTimeFormatter.hpp"
#include "../../include/ICU4XDateTimeFormatter.hpp"
#include "../../include/ICU4XTimeFormatter.hpp"
#include "../../include/ICU4XDataStruct.hpp"
#include "../../include/ICU4XLogger.hpp"
#include "../../include/ICU4XCustomTimeZone.hpp"

#include <atomic>
#include <iostream>
#include <array>

int main() {
    ICU4XLogger::init_simple_logger();
    ICU4XLocale locale = ICU4XLocale::create("es").ok().value();
    std::cout << "Running test for locale " << locale.tostring().ok().value() << std::endl;
    ICU4XDataProvider dp = ICU4XDataProvider::create_test();

    ICU4XGregorianDateTime date = ICU4XGregorianDateTime::try_new(2022, 07, 11, 13, 06, 42).ok().value();

    ICU4XTimeFormatter tf = ICU4XTimeFormatter::try_new(dp, locale, ICU4XTimeLength::Short).ok().value();
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

    ICU4XGregorianDateTimeFormatter dtf = ICU4XGregorianDateTimeFormatter::try_new(dp, locale, ICU4XDateLength::Medium, ICU4XTimeLength::Short).ok().value();
    out = dtf.format_datetime(date).ok().value();
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "11 jul 2022, 13:06") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    locale = ICU4XLocale::create("en-u-ca-japanese").ok().value();
    ICU4XCalendar cal = ICU4XCalendar::try_new_for_locale(dp, locale).ok().value();
    ICU4XDateTime any_date = ICU4XDateTime::try_new_from_iso_in_calendar(2020, 10, 5, 13, 33, 15, cal).ok().value();
    ICU4XDateTimeFormatter any_dtf = ICU4XDateTimeFormatter::try_new(dp, locale, ICU4XDateLength::Medium, ICU4XTimeLength::Short).ok().value();
    out = any_dtf.format_datetime(any_date).ok().value();
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "Oct 5, 2 Reiwa, 1:33 PM") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    ICU4XCustomTimeZone time_zone = ICU4XCustomTimeZone::create_from_str("-6:00").ok().value();
    ICU4XMetaZoneCalculator mzcalc = ICU4XMetaZoneCalculator::try_new(dp).ok().value();
    time_zone.try_set_time_zone_id("uschi").ok().value();
    std::string time_zone_id_return = time_zone.time_zone_id().ok().value();
    if (time_zone_id_return != "uschi") {
        std::cout << "Time zone ID does not roundtrip" << std::endl;
        return 1;
    }
    ICU4XIsoDateTime local_datetime = ICU4XIsoDateTime::try_new(2022, 8, 25, 0, 0, 0).ok().value();
    time_zone.maybe_calculate_meta_zone(local_datetime, mzcalc);
    std::string meta_zone_id_return = time_zone.meta_zone_id().ok().value();
    if (meta_zone_id_return != "amce") {
        std::cout << "Meta zone ID not calculated correctly; got " << meta_zone_id_return << std::endl;
        return 1;
    }
    // TODO: Pass the CustomTimeZone into a ZonedDateTimeFormatter once added to FFI

    return 0;
}
