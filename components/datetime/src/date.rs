#[derive(Default, Debug)]
pub struct DateTime {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
    pub millisecond: usize,
}

impl DateTime {
    pub fn new(
        year: usize,        // 0-
        month: usize,       // 0-11
        day: usize,         // 0-31
        hour: usize,        // 0-23
        minute: usize,      // 0-60
        second: usize,      // 0-60
        millisecond: usize, // 0-999
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
        }
    }
}
