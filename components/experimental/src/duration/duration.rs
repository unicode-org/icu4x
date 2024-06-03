/// Represents a duration of time (intuitively, how long something took / will take).
/// Can be constructed ergonomically using the [`Default`](core::default::Default) trait like so:
/// ```rust
/// let duration = Duration {
///     years: 1,
///     months: 2,
///     weeks: 3,
///     ..Default::default()
/// };
/// ```

#[allow(clippy::exhaustive_structs)] // this type should be stable (and is intended to be constructed manually)
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    /// Whether the duration is positive.
    pub sign: DurationSign,
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

/// Describes whether a [`Duration`](Duration) is positive or negative.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DurationSign {
    #[default]
    /// A positive duration.
    Positive,

    /// A negative duration.
    Negative,
}

impl Duration {
    /// Create a new positive [`Duration`](Duration) with all fields set to 0.
    pub fn new() -> Self {
        Self::default()
    }
}
