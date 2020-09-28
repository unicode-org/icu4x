/// Temporary trait used to represent the input data for `DateTimeFormat`.
///
/// This type represents all data that the formatted needs in order to produced formatted string.
///
/// *Note*: At the moment we support only `gregorian` calendar, and plan to extend support to
/// other calendars in the upcoming releases.
pub trait DateTimeType {
    fn year(&self) -> usize;
    fn month(&self) -> usize;
    fn day(&self) -> usize;
    fn hour(&self) -> usize;
    fn minute(&self) -> usize;
    fn second(&self) -> usize;
    fn millisecond(&self) -> usize;
}
/// Temporary implementation of `DateTimeType`,
/// which is used in tests, benchmarks and examples of this component.
///
/// # Examples
///
/// ```
/// use icu_datetime::DummyDateTime;
///
/// let dt = DummyDateTime::new(2020, 9, 24, 13, 21, 0, 0);
/// ```
#[derive(Debug, Default)]
pub struct DummyDateTime {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
    pub millisecond: usize,
}

impl DummyDateTime {
    /// Constructor for the `DummyDateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_datetime::DummyDateTime;
    ///
    /// let dt = DummyDateTime::new(2020, 9, 24, 13, 21, 0, 0);
    /// ```
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

impl DateTimeType for DummyDateTime {
    fn year(&self) -> usize {
        self.year
    }
    fn month(&self) -> usize {
        self.month
    }
    fn day(&self) -> usize {
        self.day
    }
    fn hour(&self) -> usize {
        self.hour
    }
    fn minute(&self) -> usize {
        self.minute
    }
    fn second(&self) -> usize {
        self.second
    }
    fn millisecond(&self) -> usize {
        self.millisecond
    }
}
