use icu_calendar::error::DateFromFieldsError;
use icu_calendar::types::DateFields;
use icu_calendar::{AnyCalendar, AnyCalendarKind, Date};

fn main() -> Result<(), DateFromFieldsError> {
    let cal = AnyCalendar::new(AnyCalendarKind::Iso);

    let mut fields = DateFields::default();

    fields.extended_year = Some(2024);
    fields.month_code = Some(b"M03");
    fields.day = Some(15);

    let date = Date::try_from_fields(fields, Default::default(), cal)?;

    println!("Constructed date = {:?}", date);

    Ok(())
}
