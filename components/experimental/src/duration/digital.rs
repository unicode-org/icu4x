use fixed_decimal::FixedDecimal;

struct FixedUnsignedInteger {
    value: u32,
}

struct FixedUnsignedDecimal {
    value: FixedDecimal,
}

enum DigitalDurationUnits {
    HourMinute(FixedUnsignedInteger, FixedUnsignedInteger),
    MinuteSecond(FixedUnsignedInteger, FixedUnsignedDecimal),
    HourMinuteSecond(
        FixedUnsignedInteger,
        FixedUnsignedInteger,
        FixedUnsignedDecimal,
    ),
}

struct DigitalDurationFormatter {}

impl DigitalDurationFormatter {
    pub fn format(&self, sign: fixed_decimal::Sign, value: DigitalDurationUnits) {
        match value {
            DigitalDurationUnits::HourMinute(hour, minute) => {
                // Format the hour and minute
                todo!()
            }
            DigitalDurationUnits::MinuteSecond(minute, second) => {
                // Format the minute and second
                todo!()
            }
            DigitalDurationUnits::HourMinuteSecond(hour, minute, second) => {
                // Format the hour, minute, and second
                todo!()
            }
        }
    }
}
