// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

fn main() {
    println!("resolved backend: {:?}", icu_host_info::resolved_backend());
    println!("-----");
    println!(
        "requested locales: {:?}",
        icu_host_info::requested_locales()
    );
    println!("calendar: {:?}", icu_host_info::calendar());
    println!("region: {:?}", icu_host_info::region());
    println!("hour_cycle: {:?}", icu_host_info::hour_cycle());
    println!(
        "measurement_system: {:?}",
        icu_host_info::measurement_system()
    );
    println!(
        "measurement_unit_override: {:?}",
        icu_host_info::measurement_unit_override()
    );
    println!("first_day: {:?}", icu_host_info::first_day_of_week());
    println!("collation: {:?}", icu_host_info::collation());
    println!("-----");
    println!(
        "unicode_extensions: {:?}",
        icu_host_info::unicode_extensions().unwrap().to_string()
    );
    #[cfg(feature = "datetime")]
    println!(
        "datetimeformatter_preferences: {:#?}",
        icu_host_info::datetime_preferences()
    );
}
