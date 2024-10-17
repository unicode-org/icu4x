use icu_calendar::{Date, Gregorian, Time};
use icu_datetime::{
    neo_marker::{NeoTimeZoneGenericMarker, NeoTimeZoneLocationMarker, NeoTimeZoneSpecificMarker},
    NeoSkeletonLength, TypedNeoFormatter,
};
use icu_locale_core::locale;
use icu_timezone::{TimeZoneIdMapper, TimeZoneInfo, ZoneVariant};
use writeable::adapters::LossyWrap;

fn main() {
    let date = Date::try_new_gregorian(2024, 10, 17).unwrap();
    let time = Time::midnight();

    let location = TypedNeoFormatter::<Gregorian, _>::try_new(
        &locale!("en-GB").into(),
        NeoTimeZoneLocationMarker::with_length(NeoSkeletonLength::Long),
    )
    .unwrap();

    let generic = TypedNeoFormatter::<Gregorian, _>::try_new(
        &locale!("en-GB").into(),
        NeoTimeZoneGenericMarker::with_length(NeoSkeletonLength::Long),
    )
    .unwrap();

    let specific = TypedNeoFormatter::<Gregorian, _>::try_new(
        &locale!("en-GB").into(),
        NeoTimeZoneSpecificMarker::with_length(NeoSkeletonLength::Long),
    )
    .unwrap();

    for time_zone_id in TimeZoneIdMapper::new().as_borrowed().iter_canonical() {
        let zone = TimeZoneInfo {
            time_zone_id,
            local_time: Some((date.to_iso(), time)),
            offset: None,
            zone_variant: None
        };

        println!(
            "{}: {}, {}, {}",
            &*zone.time_zone_id,
            LossyWrap(location.format(&zone)),
            LossyWrap(generic.format(&zone)),
            LossyWrap(specific.format(&TimeZoneInfo { zone_variant: Some(ZoneVariant::standard()), ..zone})),
        );
    }

    println!();

    for time_zone_id in TimeZoneIdMapper::new().as_borrowed().iter_metazones() {
        let zone = TimeZoneInfo {
            time_zone_id,
            local_time: Some((date.to_iso(), time)),
            offset: None,
            zone_variant: None
        };

        println!(
            "{}, {}",
            LossyWrap(generic.format(&zone)),
            LossyWrap(specific.format(&TimeZoneInfo { zone_variant: Some(ZoneVariant::standard()), ..zone})),
        );
    }
}
