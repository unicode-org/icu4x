pub enum RetrievalError {
    /// Unable to retrieve the locale
    NullLocale,

    /// Unable to retrieve the calendar
    NullCalendar,

    /// Received NULL Pointer
    NullPointer,

    /// Error converting into `&CStr` to `&str`
    ConversionError,

    /// UnknownCategory when retrieving locale for linux
    UnknownCategory,
}
