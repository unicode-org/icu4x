#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// Represents a duration of time.
pub struct Duration {
    /// The number of years in the duration.
    pub years: u64,
    /// The number of months in the duration.
    pub months: u64,
    /// The number of weeks in the duration.
    pub weeks: u64,
    /// The number of days in the duration.
    pub days: u64,
    /// The number of hours in the duration.
    pub hours: u64,
    /// The number of minutes in the duration.
    pub minutes: u64,
    /// The number of seconds in the duration.
    pub seconds: u64,
    /// The number of milliseconds in the duration.
    pub milliseconds: u64,
    /// The number of microseconds in the duration.
    pub microseconds: u64,
    /// The number of nanoseconds in the duration.
    pub nanoseconds: u64,
}

impl Duration {
    /// Create a new [`Duration`](Duration) with all fields set to 0.
    pub fn new() -> Self {
        Self::default()
    }
}
