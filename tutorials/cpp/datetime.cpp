// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "ICU4XGregorianDateFormatter.hpp"
#include "ICU4XGregorianDateTimeFormatter.hpp"
#include "ICU4XDateTimeFormatter.hpp"
#include "ICU4XTimeFormatter.hpp"
#include "ICU4XLogger.hpp"
#include "ICU4XCustomTimeZone.hpp"
#include "ICU4XTimeZoneIdMapper.hpp"
#include "ICU4XTimeZoneIdMapperWithFastCanonicalization.hpp"
#include "ICU4XGregorianZonedDateTimeFormatter.hpp"
#include "ICU4XZonedDateTimeFormatter.hpp"

#include <atomic>
#include <iostream>
#include <array>

int main() {
    ICU4XLogger::init_simple_logger();
    std::unique_ptr<ICU4XLocale> locale = ICU4XLocale::create_from_string("es").ok().value();
    std::cout << "Running test for locale " << locale->to_string() << std::endl;
    std::unique_ptr<ICU4XDataProvider> dp = ICU4XDataProvider::create_compiled();

    std::unique_ptr<ICU4XIsoDateTime> date = ICU4XIsoDateTime::create(2022, 07, 11, 13, 06, 42, 0).ok().value();

    std::unique_ptr<ICU4XTimeFormatter> tf = ICU4XTimeFormatter::create_with_length(*dp.get(), *locale.get(), ICU4XTimeLength::Short).ok().value();
    std::string out = tf->format_iso_datetime(*date.get());
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "13:06") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::unique_ptr<ICU4XGregorianDateFormatter> df = ICU4XGregorianDateFormatter::create_with_length(*dp.get(), *locale.get(), ICU4XDateLength::Full).ok().value();
    out = df->format_iso_datetime(*date.get());
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "lunes, 11 de julio de 2022") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::unique_ptr<ICU4XGregorianDateTimeFormatter> dtf = ICU4XGregorianDateTimeFormatter::create_with_lengths(*dp.get(), *locale.get(), ICU4XDateLength::Medium, ICU4XTimeLength::Short).ok().value();
    out = dtf->format_iso_datetime(*date.get());
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "11 jul 2022, 13:06") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    locale = ICU4XLocale::create_from_string("en-u-ca-japanese").ok().value();
    std::unique_ptr<ICU4XCalendar> cal = ICU4XCalendar::create_for_locale(*dp.get(), *locale.get()).ok().value();
    std::unique_ptr<ICU4XDateTime> any_date = ICU4XDateTime::create_from_iso_in_calendar(2020, 10, 5, 13, 33, 15, 0, *cal.get()).ok().value();
    std::unique_ptr<ICU4XDateTimeFormatter> any_dtf = ICU4XDateTimeFormatter::create_with_lengths(*dp.get(), *locale.get(), ICU4XDateLength::Medium, ICU4XTimeLength::Short).ok().value();
    out = any_dtf->format_datetime(*any_date.get()).ok().value();
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "Oct 5, 2 Reiwa, 1:33\u202fPM") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::unique_ptr<ICU4XCustomTimeZone> time_zone = ICU4XCustomTimeZone::create_from_string("-06:00").ok().value();
    int32_t offset = time_zone->gmt_offset_seconds().value();
    if (offset != -21600) {
        std::cout << "GMT offset doesn't parse" << std::endl;
        return 1;
    }
    std::unique_ptr<ICU4XMetazoneCalculator> mzcalc = ICU4XMetazoneCalculator::create(*dp.get()).ok().value();
    std::unique_ptr<ICU4XTimeZoneIdMapper> mapper = ICU4XTimeZoneIdMapper::create(*dp.get()).ok().value();
    time_zone->try_set_iana_time_zone_id(*mapper.get(), "america/chicago").ok().value();
    std::string time_zone_id_return = time_zone->time_zone_id().value();
    if (time_zone_id_return != "uschi") {
        std::cout << "Time zone ID does not roundtrip: " << time_zone_id_return << std::endl;
        return 1;
    }
    std::string normalized_iana_id = mapper->normalize_iana("America/CHICAGO").ok().value().ok().value();
    if (normalized_iana_id != "America/Chicago") {
        std::cout << "Time zone ID does not normalize: " << normalized_iana_id << std::endl;
        return 1;
    }
    std::string canonicalied_iana_id = mapper->canonicalize_iana("Asia/Calcutta").ok().value().ok().value();
    if (canonicalied_iana_id != "Asia/Kolkata") {
        std::cout << "Time zone ID does not canonicalize: " << canonicalied_iana_id << std::endl;
        return 1;
    }
    std::string slow_recovered_iana_id = mapper->find_canonical_iana_from_bcp47("uschi").ok().value();
    if (slow_recovered_iana_id != "America/Chicago") {
        std::cout << "Time zone ID does not roundtrip (slow): " << slow_recovered_iana_id << std::endl;
        return 1;
    }
    std::unique_ptr<ICU4XTimeZoneIdMapperWithFastCanonicalization> reverse_mapper = ICU4XTimeZoneIdMapperWithFastCanonicalization::create(*dp.get()).ok().value();
    std::string fast_recovered_iana_id = reverse_mapper->canonical_iana_from_bcp47("uschi").ok().value();
    if (fast_recovered_iana_id != "America/Chicago") {
        std::cout << "Time zone ID does not roundtrip (fast): " << fast_recovered_iana_id << std::endl;
        return 1;
    }
    std::unique_ptr<ICU4XIsoDateTime> local_datetime = ICU4XIsoDateTime::create(2022, 8, 25, 0, 0, 0, 0).ok().value();
    time_zone->maybe_calculate_metazone(*mzcalc.get(), *local_datetime.get());
    std::string metazone_id_return = time_zone->metazone_id().value();
    if (metazone_id_return != "amce") {
        std::cout << "Metazone ID not calculated correctly; got " << metazone_id_return << std::endl;
        return 1;
    }
    // Note: The daylight time switch should normally come from TZDB calculations.
    time_zone->set_daylight_time();
    std::string zone_variant_return = time_zone->zone_variant().value();
    if (zone_variant_return != "dt") {
        std::cout << "Zone variant not calculated correctly; got " << zone_variant_return << std::endl;
        return 1;
    }

    std::unique_ptr<ICU4XGregorianZonedDateTimeFormatter> gzdtf = ICU4XGregorianZonedDateTimeFormatter::create_with_lengths(*dp.get(), *locale.get(), ICU4XDateLength::Full, ICU4XTimeLength::Full).ok().value();
    out = gzdtf->format_iso_datetime_with_custom_time_zone(*date.get(), *time_zone.get());
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "Monday, July 11, 2022, 1:06:42\u202fPM Central Daylight Time") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::unique_ptr<ICU4XZonedDateTimeFormatter> zdtf = ICU4XZonedDateTimeFormatter::create_with_lengths(*dp.get(), *locale.get(), ICU4XDateLength::Full, ICU4XTimeLength::Full).ok().value();
    out = zdtf->format_datetime_with_custom_time_zone(*any_date.get(), *time_zone.get()).ok().value();
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "Monday, October 5, 2 Reiwa, 1:33:15\u202fPM Central Daylight Time") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    return 0;
}
