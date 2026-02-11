// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/DateFormatter.hpp>
#include <icu4x/DateFormatterGregorian.hpp>
#include <icu4x/DateTimeFormatter.hpp>
#include <icu4x/DateTimeFormatterGregorian.hpp>
#include <icu4x/TimeFormatter.hpp>
#include <icu4x/ZonedDateFormatter.hpp>
#include <icu4x/ZonedTimeFormatter.hpp>
#include <icu4x/ZonedDateTimeFormatter.hpp>
#include <icu4x/TimeZoneFormatter.hpp>
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
    std::string out;

    std::unique_ptr<DateFormatter> fmt_d = DateFormatter::create_d(*locale.get(), DateTimeLength::Medium, std::nullopt).ok().value();
    out = fmt_d->format_iso(*date.get());
    std::cout << "Fieldset D: " << out;
    if (out != "11") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateFormatter> fmt_md = DateFormatter::create_md(*locale.get(), DateTimeLength::Medium, std::nullopt).ok().value();
    out = fmt_md->format_iso(*date.get());
    std::cout << "Fieldset MD: " << out;
    if (out != "11 jul") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateFormatter> fmt_ymd = DateFormatter::create_ymd(*locale.get(), DateTimeLength::Medium, std::nullopt, std::nullopt).ok().value();
    out = fmt_ymd->format_iso(*date.get());
    std::cout << "Fieldset YMD: " << out;
    if (out != "11 jul 2022") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateFormatter> fmt_de = DateFormatter::create_de(*locale.get(), DateTimeLength::Medium, std::nullopt).ok().value();
    out = fmt_de->format_iso(*date.get());
    std::cout << "Fieldset DE: " << out;
    if (out != "lun 11") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateFormatter> fmt_mde = DateFormatter::create_mde(*locale.get(), DateTimeLength::Medium, std::nullopt).ok().value();
    out = fmt_mde->format_iso(*date.get());
    std::cout << "Fieldset MDE: " << out;
    if (out != "lun, 11 jul") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateFormatter> fmt_ymde = DateFormatter::create_ymde(*locale.get(), DateTimeLength::Medium, std::nullopt, std::nullopt).ok().value();
    out = fmt_ymde->format_iso(*date.get());
    std::cout << "Fieldset YMDE: " << out;
    if (out != "lun, 11 jul 2022") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateFormatter> fmt_e = DateFormatter::create_e(*locale.get(), DateTimeLength::Medium).ok().value();
    out = fmt_e->format_iso(*date.get());
    std::cout << "Fieldset E: " << out;
    if (out != "lun") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateFormatter> fmt_m = DateFormatter::create_m(*locale.get(), DateTimeLength::Medium, std::nullopt).ok().value();
    out = fmt_m->format_iso(*date.get());
    std::cout << "Fieldset M: " << out;
    if (out != "jul") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateFormatter> fmt_ym = DateFormatter::create_ym(*locale.get(), DateTimeLength::Medium, std::nullopt, std::nullopt).ok().value();
    out = fmt_ym->format_iso(*date.get());
    std::cout << "Fieldset YM: " << out;
    if (out != "jul 2022") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateFormatter> fmt_y = DateFormatter::create_y(*locale.get(), DateTimeLength::Medium, std::nullopt, std::nullopt).ok().value();
    out = fmt_y->format_iso(*date.get());
    std::cout << "Fieldset Y: " << out;
    if (out != "2022") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<TimeFormatter> fmt_t = TimeFormatter::create(*locale.get(), std::nullopt, TimePrecision::Minute, std::nullopt).ok().value();
    out = fmt_t->format(*time.get());
    std::cout << "Fieldset T: " << out;
    if (out != "13:06") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateTimeFormatter> fmt_dt = DateTimeFormatter::create_dt(*locale.get(), DateTimeLength::Medium, TimePrecision::Minute, std::nullopt).ok().value();
    out = fmt_dt->format_iso(*date.get(), *time.get());
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

    std::unique_ptr<DateFormatterGregorian> fmt_ymd_g = DateFormatterGregorian::create_ymd(*locale.get(), DateTimeLength::Long, std::nullopt, std::nullopt).ok().value();
    out = fmt_ymd_g->format_iso(*date.get());
    std::cout << "Formatted value is " << out;
    if (out != "11 de julio de 2022") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<DateTimeFormatterGregorian> fmt_ymdt_g = DateTimeFormatterGregorian::create_ymdt(*locale.get(), DateTimeLength::Medium, TimePrecision::Minute, std::nullopt, std::nullopt).ok().value();
    out = fmt_ymdt_g->format_iso(*date.get(), *time.get());
    std::cout << "Formatted value is " << out;
    if (out != "11 jul 2022, 13:06") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<TimeZone> time_zone = TimeZone::from_iana_id("america/chicago");

    std::unique_ptr<UtcOffset> utc_offset = UtcOffset::from_string("-05:00").ok().value();
    if (utc_offset->seconds() != -18000) {
        std::cout << "UTC offset doesn't parse" << std::endl;
        return 1;
    }

    std::unique_ptr<TimeZoneInfo> time_zone_info = time_zone->with_offset(*utc_offset.get())->at_date_time_iso(*date.get(), *time.get());

    std::unique_ptr<TimeZoneFormatter> fmt_generic_short = TimeZoneFormatter::create_generic_short(*locale.get()).ok().value();
    out = fmt_generic_short->format(*time_zone_info.get()).ok().value();
    std::cout << "Fieldset Z Generic Short: " << out;
    if (out != "hora de Chicago") {
        // note: this falls back to Generic Location
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<ZonedDateFormatter> fmt_md_generic_short = ZonedDateFormatter::create_generic_short(*locale.get(), *fmt_md).ok().value();
    out = fmt_md_generic_short->format_iso(*date.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MD Generic Short: " << out;
    if (out != "11 jul hora de Chicago") {
        // note: this falls back to Generic Location
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<ZonedTimeFormatter> fmt_t_specific_short = ZonedTimeFormatter::create_specific_short(*locale.get(), std::nullopt, TimePrecision::Minute, std::nullopt).ok().value();
    out = fmt_t_specific_short->format(*time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset T Specific Short: " << out;
    if (out != "13:06 GMT-5") {
        // note: this falls back to Localized Offset
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<ZonedDateTimeFormatter> fmt_mdt_generic_short = ZonedDateTimeFormatter::create_generic_short(*locale.get(), *fmt_mdt).ok().value();
    out = fmt_mdt_generic_short->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Generic Short: " << out;
    if (out != "11 jul, 13:06 hora de Chicago") {
        // note: this falls back to Generic Location
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<ZonedDateTimeFormatter> fmt_mdt_generic_long = ZonedDateTimeFormatter::create_generic_long(*locale.get(), *fmt_mdt).ok().value();
    out = fmt_mdt_generic_long->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Generic Long: " << out;
    if (out != "11 jul, 13:06 hora central") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<ZonedDateTimeFormatter> fmt_mdt_specific_short = ZonedDateTimeFormatter::create_specific_short(*locale.get(), *fmt_mdt).ok().value();
    out = fmt_mdt_specific_short->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Specific Short: " << out;
    if (out != "11 jul, 13:06 GMT-5") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<ZonedDateTimeFormatter> fmt_mdt_specific_long = ZonedDateTimeFormatter::create_specific_long(*locale.get(), *fmt_mdt).ok().value();
    out = fmt_mdt_specific_long->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Specific Long: " << out;
    if (out != "11 jul, 13:06 hora de verano central") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<ZonedDateTimeFormatter> fmt_mdt_localized_offset_short = ZonedDateTimeFormatter::create_localized_offset_short(*locale.get(), *fmt_mdt).ok().value();
    out = fmt_mdt_localized_offset_short->format_iso(*date.get(), *time.get(), *time_zone_info.get()).ok().value();
    std::cout << "Fieldset MDT Localized Offset Short: " << out;
    if (out != "11 jul, 13:06 GMT-5") {
        std::cout << " (unexpected!)";
        saw_unexpected_output = true;
    }
    std::cout << std::endl;

    std::unique_ptr<ZonedDateTimeFormatter> fmt_mdt_localized_offset_long = ZonedDateTimeFormatter::create_localized_offset_long(*locale.get(), *fmt_mdt).ok().value();
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
