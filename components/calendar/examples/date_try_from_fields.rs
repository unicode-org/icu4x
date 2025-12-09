// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main]

use icu_calendar::error::DateFromFieldsError;
use icu_calendar::types::DateFields;
use icu_calendar::{AnyCalendar, AnyCalendarKind, Date};

#[no_mangle]
pub extern "C" fn main() {
    real_main().unwrap();
}

fn real_main() -> Result<(), DateFromFieldsError> {
    let cal = AnyCalendar::new(AnyCalendarKind::Iso);

    let mut fields = DateFields::default();
    fields.extended_year = Some(2025);
    fields.month_code = Some(b"M07");
    fields.day = Some(8);

    let date = Date::try_from_fields(fields, Default::default(), cal)?;

    println!("Constructed date = {:?}", date);

    Ok(())
}
