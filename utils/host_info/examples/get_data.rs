// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_host_info::HostInfo;

fn main() {
    println!("resolved backend: {:?}", HostInfo::resolved_backend());
    println!("-----");
    println!("requested locales: {:?}", HostInfo::requested_locales());
    println!("calendar: {:?}", HostInfo::calendar());
    println!("region: {:?}", HostInfo::region());
    println!("hour_cycle: {:?}", HostInfo::hour_cycle());
    println!("measurement_system: {:?}", HostInfo::measurement_system());
    println!(
        "measurement_unit_override: {:?}",
        HostInfo::measurement_unit_override()
    );
    println!("first_day: {:?}", HostInfo::first_day_of_week());
    println!("collation: {:?}", HostInfo::collation());
    println!("-----");
    println!(
        "unicode_extensions: {:?}",
        HostInfo::unicode_extensions().unwrap().to_string()
    );
    #[cfg(feature = "datetime")]
    println!(
        "datetimeformatter_preferences: {:#?}",
        HostInfo::datetime_preferences()
    );
}
