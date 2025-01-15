use icu::{
    calendar::Date,
    datetime::{fieldsets, DateTimeFormatter},
    locale::locale,
    timezone::{IxdtfParser, Time, TimeZoneIdMapper, UtcOffset, ZoneVariant, ZonedDateTime},
};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let ts: jiff::Timestamp = "2024-09-10T23:37:20.123456789Z".parse()?;
    let zoned: jiff::Zoned = ts.intz("Asia/Tokyo")?;

    // Convert to ICU types
    let date = Date::try_new_iso(
        i32::from(zoned.year()),
        zoned.month().unsigned_abs(),
        zoned.day().unsigned_abs(),
    )?;

    let time = Time::try_new(
        zoned.hour().unsigned_abs(),
        zoned.minute().unsigned_abs(),
        zoned.second().unsigned_abs(),
        u32::from(zoned.millisecond().unsigned_abs()) * 1_000_000
            + u32::from(zoned.microsecond().unsigned_abs()) * 1_000
            + u32::from(zoned.nanosecond().unsigned_abs()),
    )?;

    let zone = 
        // ICU uses BCP47 time zone IDs
        TimeZoneIdMapper::new().iana_to_bcp47(zoned.time_zone().iana_name().unwrap_or("Etc/Unknown"))
        // In ICU's model, a time zone has a fixed offset, as that's required for formatting
        .with_offset(UtcOffset::try_from_seconds(zoned.offset().seconds()).ok())
        // Display names might change over time for a given zone (e.g. it might change from Eastern Time to
        // Central Time), so the ICU timezone needs a reference date and time.
        .at_time((date, time))
        // And finally, the zone variant is also required for formatting
        .with_zone_variant(match zoned.time_zone().to_offset(zoned.timestamp()).1 {
            jiff::tz::Dst::Yes => ZoneVariant::Daylight,
            jiff::tz::Dst::No => ZoneVariant::Standard,
        });

    let zoned_date_time = ZonedDateTime { date, time, zone };

    // Alternatively, the ICU ZonedDateTime can be parsed from a serialized IXDTF string.
    assert_eq!(
        IxdtfParser::new()
            .try_iso_from_str(&zoned.to_string())
            .unwrap(),
        zoned_date_time
    );

    // Preferences for an English formatter using the Japanese calendar
    let prefs = locale!("en-GB-u-ca-japanese").into();

    // A medium-length year-month-day-time-specific-zone formatter
    let formatter =
        DateTimeFormatter::try_new(prefs, fieldsets::YMDT::medium().with_zone_specific_long())?;

    println!("{}", formatter.format_any_calendar(&zoned_date_time)); // 11 Sept 6 Reiwa, 08:37:20 Japan Standard Time

    Ok(())
}
