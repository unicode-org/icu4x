``datetime::ffi``
=================

.. js:class:: ICU4XDateTime

    An ICU4X DateTime object capable of containing a date and time for any calendar.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. js:staticfunction:: try_new_from_iso_in_calendar(year, month, day, hour, minute, second, nanosecond, calendar)

        Creates a new :js:class:`ICU4XDateTime` representing the ISO date and time given but in a given calendar

        See the `Rust documentation for new_iso_datetime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_iso_datetime>`__ for more information.


    .. js:staticfunction:: try_new_from_codes_in_calendar(era_code, year, month_code, day, hour, minute, second, nanosecond, calendar)

        Creates a new :js:class:`ICU4XDateTime` representing the ISO date and time given but in a given calendar

        See the `Rust documentation for new_from_codes <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_from_codes>`__ for more information.


    .. js:staticfunction:: new_from_date_and_time(date, time)

        Creates a new :js:class:`ICU4XDateTime` from an :js:class:`ICU4XDate` and :js:class:`ICU4XTime` object

        See the `Rust documentation for new <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new>`__ for more information.


    .. js:function:: date()

        Gets a copy of the date contained in this object

        See the `Rust documentation for date <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.date>`__ for more information.


    .. js:function:: time()

        Gets the time contained in this object

        See the `Rust documentation for time <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time>`__ for more information.


    .. js:function:: to_iso()

        Converts this date to ISO

        See the `Rust documentation for to_iso <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_iso>`__ for more information.


    .. js:function:: to_calendar(calendar)

        Convert this datetime to one in a different calendar

        See the `Rust documentation for to_calendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_calendar>`__ for more information.


.. js:class:: ICU4XGregorianDateTime

    An ICU4X DateTime object capable of containing a Gregorian date and time.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. js:staticfunction:: try_new(year, month, day, hour, minute, second, nanosecond)

        Creates a new :js:class:`ICU4XGregorianDateTime` from the specified date and time.

        See the `Rust documentation for new_gregorian_datetime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime>`__ for more information.


    .. js:function:: time()

        Gets the time contained in this object

        See the `Rust documentation for time <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time>`__ for more information.


    .. js:function:: to_any()

        Converts this to an :js:class:`ICU4XDateTime` capable of being mixed with dates of other calendars

        See the `Rust documentation for to_any <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_any>`__ for more information.


.. js:class:: ICU4XIsoDateTime

    An ICU4X DateTime object capable of containing a ISO-8601 date and time.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. js:staticfunction:: try_new(year, month, day, hour, minute, second, nanosecond)

        Creates a new :js:class:`ICU4XIsoDateTime` from the specified date and time.

        See the `Rust documentation for new_gregorian_datetime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime>`__ for more information.


    .. js:staticfunction:: new_from_date_and_time(date, time)

        Creates a new :js:class:`ICU4XIsoDateTime` from an :js:class:`ICU4XIsoDate` and :js:class:`ICU4XTime` object

        See the `Rust documentation for new <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new>`__ for more information.


    .. js:staticfunction:: from_minutes_since_local_unix_epoch(minutes)

        Construct from the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)

        See the `Rust documentation for from_minutes_since_local_unix_epoch <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.from_minutes_since_local_unix_epoch>`__ for more information.


    .. js:function:: date()

        Gets the date contained in this object

        See the `Rust documentation for date <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.date>`__ for more information.


    .. js:function:: time()

        Gets the time contained in this object

        See the `Rust documentation for time <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#structfield.time>`__ for more information.


    .. js:function:: to_any()

        Converts this to an :js:class:`ICU4XDateTime` capable of being mixed with dates of other calendars

        See the `Rust documentation for to_any <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.to_any>`__ for more information.


    .. js:function:: minutes_since_local_unix_epoch()

        Gets the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)

        See the `Rust documentation for minutes_since_local_unix_epoch <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.minutes_since_local_unix_epoch>`__ for more information.

