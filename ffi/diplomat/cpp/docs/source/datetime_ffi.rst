``datetime::ffi``
=================

.. cpp:class:: ICU4XDateTime

    An ICU4X DateTime object capable of containing a date and time for any calendar.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XDateTime, ICU4XError> try_new_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar& calendar)

        Creates a new :cpp:class:`ICU4XDateTime` representing the ISO date and time given but in a given calendar

        See the `Rust documentation for new_iso_datetime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_iso_datetime>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XDateTime, ICU4XError> try_new_from_codes_in_calendar(const std::string_view era_code, int32_t year, const std::string_view month_code, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond, const ICU4XCalendar& calendar)

        Creates a new :cpp:class:`ICU4XDateTime` representing the ISO date and time given but in a given calendar

        See the `Rust documentation for new_from_codes <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_from_codes>`__ for more information.


    .. cpp:function:: static ICU4XDateTime new_from_date_and_time(const ICU4XDate& date, const ICU4XTime& time)

        Creates a new :cpp:class:`ICU4XDateTime` from an :cpp:class:`ICU4XDate` and :cpp:class:`ICU4XTime` object

        See the `Rust documentation for new <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new>`__ for more information.


    .. cpp:function:: ICU4XDate date() const

        Gets a copy of the date contained in this object

        See the `Rust documentation for date <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.date>`__ for more information.


    .. cpp:function:: ICU4XTime time() const

        Gets the time contained in this object

        See the `Rust documentation for time <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time>`__ for more information.


    .. cpp:function:: ICU4XIsoDateTime to_iso() const

        Converts this date to ISO

        See the `Rust documentation for to_iso <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_iso>`__ for more information.


    .. cpp:function:: ICU4XDateTime to_calendar(const ICU4XCalendar& calendar) const

        Convert this datetime to one in a different calendar

        See the `Rust documentation for to_calendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_calendar>`__ for more information.


.. cpp:class:: ICU4XGregorianDateTime

    An ICU4X DateTime object capable of containing a Gregorian date and time.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XGregorianDateTime, ICU4XError> try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond)

        Creates a new :cpp:class:`ICU4XGregorianDateTime` from the specified date and time.

        See the `Rust documentation for new_gregorian_datetime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime>`__ for more information.


    .. cpp:function:: ICU4XTime time() const

        Gets the time contained in this object

        See the `Rust documentation for time <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time>`__ for more information.


    .. cpp:function:: ICU4XDateTime to_any() const

        Converts this to an :cpp:class:`ICU4XDateTime` capable of being mixed with dates of other calendars

        See the `Rust documentation for to_any <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_any>`__ for more information.


.. cpp:class:: ICU4XIsoDateTime

    An ICU4X DateTime object capable of containing a ISO-8601 date and time.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XIsoDateTime, ICU4XError> try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond)

        Creates a new :cpp:class:`ICU4XIsoDateTime` from the specified date and time.

        See the `Rust documentation for new_gregorian_datetime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime>`__ for more information.


    .. cpp:function:: static ICU4XIsoDateTime new_from_date_and_time(const ICU4XIsoDate& date, const ICU4XTime& time)

        Creates a new :cpp:class:`ICU4XIsoDateTime` from an :cpp:class:`ICU4XIsoDate` and :cpp:class:`ICU4XTime` object

        See the `Rust documentation for new <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XIsoDateTime, ICU4XError> from_minutes_since_local_unix_epoch(int32_t minutes)

        Construct from the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)

        See the `Rust documentation for from_minutes_since_local_unix_epoch <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.from_minutes_since_local_unix_epoch>`__ for more information.


    .. cpp:function:: ICU4XIsoDate date() const

        Gets the date contained in this object

        See the `Rust documentation for date <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.date>`__ for more information.


    .. cpp:function:: ICU4XTime time() const

        Gets the time contained in this object

        See the `Rust documentation for time <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time>`__ for more information.


    .. cpp:function:: ICU4XDateTime to_any() const

        Converts this to an :cpp:class:`ICU4XDateTime` capable of being mixed with dates of other calendars

        See the `Rust documentation for to_any <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_any>`__ for more information.


    .. cpp:function:: int32_t minutes_since_local_unix_epoch() const

        Gets the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)

        See the `Rust documentation for minutes_since_local_unix_epoch <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.minutes_since_local_unix_epoch>`__ for more information.

