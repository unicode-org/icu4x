// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/GregorianDateFormatter.hpp>
#include <icu4x/DateTimeFormatter.hpp>
#include <icu4x/DateTimeFormatterGregorian.hpp>
#include <icu4x/NoCalendarFormatter.hpp>
#include <icu4x/Logger.hpp>
#include <icu4x/TimeZoneInfo.hpp>
#include <icu4x/IanaParser.hpp>

#include <atomic>
#include <iostream>
#include <array>
#include <optional>

using namespace icu4x;

int main() {
    Logger::init_simple_logger();
    std::unique_ptr<Locale> locale = Locale::from_string("es").ok().value();
    std::cout << "Running test for locale " << locale->to_string() << std::endl;

    bool saw_unexpected_output = false;

    std::unique_ptr<IsoDate> date = IsoDate::create(2022, 07, 11).ok().value();
    std::unique_ptr<Time> time = Time::create(13, 06, 42, 0).ok().value();

    std::unique_ptr<DateTimeFormatter> fmt_dt = DateTimeFormatter::create_dt(*locale.get(), DateTimeLength::Medium, TimePrecision::Minute, std::nullopt).ok().value();
    std::string out = fmt_dt->format_iso(*date.get(), *time.get());
    std::cout << "Fieldset DT: " << out;
    if (out != "11, 13:06") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateTimeFormatter> fmt_mdt = DateTimeFormatter::create_mdt(*locale.get(), DateTimeLength::Medium, TimePrecision::Minute, std::nullopt).ok().value();
    out = fmt_mdt->format_iso(*date.get(), *time.get());
    std::cout << "Fieldset MDT: " << out;
    if (out != "11 jul, 13:06") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateTimeFormatter> fmt_ymdt = DateTimeFormatter::create_ymdt(*locale.get(), DateTimeLength::Medium, TimePrecision::Minute, std::nullopt, std::nullopt).ok().value();
    out = fmt_ymdt->format_iso(*date.get(), *time.get());
    std::cout << "Fieldset YMDT: " << out;
    if (out != "11 jul 2022, 13:06") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateTimeFormatter> fmt_det = DateTimeFormatter::create_det(*locale.get(), DateTimeLength::Medium, TimePrecision::Minute, std::nullopt).ok().value();
    out = fmt_det->format_iso(*date.get(), *time.get());
    std::cout << "Fieldset DET: " << out;
    if (out != "lun 11, 13:06") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateTimeFormatter> fmt_mdet = DateTimeFormatter::create_mdet(*locale.get(), DateTimeLength::Medium, TimePrecision::Minute, std::nullopt).ok().value();
    out = fmt_mdet->format_iso(*date.get(), *time.get());
    std::cout << "Fieldset MDET: " << out;
    if (out != "lun, 11 jul, 13:06") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateTimeFormatter> fmt_ymdet = DateTimeFormatter::create_ymdet(*locale.get(), DateTimeLength::Medium, TimePrecision::Minute, std::nullopt, std::nullopt).ok().value();
    out = fmt_ymdet->format_iso(*date.get(), *time.get());
    std::cout << "Fieldset YMDET: " << out;
    if (out != "lun, 11 jul 2022, 13:06") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateTimeFormatter> fmt_et = DateTimeFormatter::create_et(*locale.get(), DateTimeLength::Medium, TimePrecision::Minute, std::nullopt).ok().value();
    out = fmt_et->format_iso(*date.get(), *time.get());
    std::cout << "Fieldset ET: " << out;
    if (out != "lun, 13:06") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<NoCalendarFormatter> tf = NoCalendarFormatter::create_with_length(*locale.get(), DateTimeLength::Short).ok().value();
    out = tf->format(*time.get());
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "13:06") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::unique_ptr<GregorianDateFormatter> df = GregorianDateFormatter::create_with_length(*locale.get(), DateTimeLength::Long).ok().value();
    out = df->format_iso(*date.get());
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "11 de julio de 2022") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::unique_ptr<DateTimeFormatterGregorian> dtf = DateTimeFormatterGregorian::create_ymdt(*locale.get(), DateTimeLength::Medium, TimePrecision::Minute, std::nullopt, std::nullopt).ok().value();
    out = dtf->format_iso(*date.get(), *time.get());
    std::cout << "Formatted value is " << out << std::endl;
    if (out != "11 jul 2022, 13:06") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::unique_ptr<IanaParser> parser = IanaParser::create();

    std::unique_ptr<TimeZone> time_zone = parser->parse("america/chicago");

    std::unique_ptr<UtcOffset> utc_offset = UtcOffset::from_string("-05:00").ok().value();
    if (utc_offset->seconds() != -18000) {
        std::cout << "UTC offset doesn't parse" << std::endl;
        return 1;
    }

    std::unique_ptr<TimeZoneInfo> time_zone_info = time_zone->with_offset(*utc_offset.get())->at_time(*date.get(), *time.get());
    
    time_zone_info->infer_zone_variant(*UtcOffsetCalculator::create().get());

    std::unique_ptr<NeoZonedDateTimeFormatter> fmt_mdt_generic_short = fmt_mdt->with_zone_generic_short(*locale.get()).ok().value();
    out = fmt_mdt_generic_short->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Generic Short: " << out;
    if (out != "11 jul, 13:06 hora de Chicago") {
        // note: this falls back to Generic Location
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<NeoZonedDateTimeFormatter> fmt_mdt_generic_long = fmt_mdt->with_zone_generic_long(*locale.get()).ok().value();
    out = fmt_mdt_generic_long->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Generic Long: " << out;
    if (out != "11 jul, 13:06 hora central") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<NeoZonedDateTimeFormatter> fmt_mdt_specific_short = fmt_mdt->with_zone_specific_short(*locale.get()).ok().value();
    out = fmt_mdt_specific_short->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Specific Short: " << out;
    if (out != "11 jul, 13:06 GMT-5") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<NeoZonedDateTimeFormatter> fmt_mdt_specific_long = fmt_mdt->with_zone_specific_long(*locale.get()).ok().value();
    out = fmt_mdt_specific_long->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Specific Long: " << out;
    if (out != "11 jul, 13:06 hora de verano central") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<NeoZonedDateTimeFormatter> fmt_mdt_localized_offset_short = fmt_mdt->with_zone_localized_offset_short(*locale.get()).ok().value();
    out = fmt_mdt_localized_offset_short->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Localized Offset Short: " << out;
    if (out != "11 jul, 13:06 GMT-5") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<NeoZonedDateTimeFormatter> fmt_mdt_localized_offset_long = fmt_mdt->with_zone_localized_offset_long(*locale.get()).ok().value();
    out = fmt_mdt_localized_offset_long->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Localized Offset Long: " << out;
    if (out != "11 jul, 13:06 GMT-05:00") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    if (saw_unexpected_output) {
        return 1;
    }

    return 0;
}
