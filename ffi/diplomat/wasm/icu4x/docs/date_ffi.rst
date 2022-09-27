``date::ffi``
=============

.. js:class:: ICU4XDate

    An ICU4X Date object capable of containing a date and time for any calendar.

    See the `Rust documentation for Date <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html>`__ for more information.


    .. js:staticfunction:: create_from_iso_in_calendar(year, month, day, calendar)

        Creates a new :js:class:`ICU4XDate` representing the ISO date and time given but in a given calendar

        See the `Rust documentation for new_from_iso <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.new_from_iso>`__ for more information.


    .. js:staticfunction:: create_from_codes_in_calendar(era_code, year, month_code, day, calendar)

        Creates a new :js:class:`ICU4XDate` from the given codes, which are interpreted in the given calendar system

        See the `Rust documentation for try_new_from_codes <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.try_new_from_codes>`__ for more information.


    .. js:function:: to_calendar(calendar)

        Convert this date to one in a different calendar

        See the `Rust documentation for to_calendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.to_calendar>`__ for more information.


    .. js:function:: to_iso()

        Converts this date to ISO

        See the `Rust documentation for to_iso <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.to_iso>`__ for more information.


    .. js:function:: day_of_month()

        Returns the 1-indexed day in the month for this date

        See the `Rust documentation for day_of_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.day_of_month>`__ for more information.


    .. js:function:: day_of_week()

        Returns the day in the week for this day

        See the `Rust documentation for day_of_week <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.day_of_week>`__ for more information.


    .. js:function:: week_of_month(first_weekday)

        Returns the week number in this month, 1-indexed, based on what is considered the first day of the week (often a locale preference).

        ``first_weekday`` can be obtained via ``first_weekday()`` on :js:class:`ICU4XWeekCalculator`

        See the `Rust documentation for week_of_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.week_of_month>`__ for more information.


    .. js:function:: week_of_year(calculator)

        Returns the week number in this year, using week data

        See the `Rust documentation for week_of_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.week_of_year>`__ for more information.


    .. js:function:: ordinal_month()

        Returns 1-indexed number of the month of this date in its year

        Note that for lunar calendars this may not lead to the same month having the same ordinal month across years; use month_code if you care about month identity.

        See the `Rust documentation for month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.month>`__ for more information.


    .. js:function:: month_code()

        Returns the month code for this date. Typically something like "M01", "M02", but can be more complicated for lunar calendars.

        See the `Rust documentation for month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.month>`__ for more information.


    .. js:function:: year_in_era()

        Returns the year number in the current era for this date

        See the `Rust documentation for year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.year>`__ for more information.


    .. js:function:: era()

        Returns the era for this date,

        See the `Rust documentation for year <https://unicode-org.github.io/icu4x-docs/doc/icu/struct.Date.html#method.year>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/types/struct.Era.html>`__


    .. js:function:: months_in_year()

        Returns the number of months in the year represented by this date

        See the `Rust documentation for months_in_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.months_in_year>`__ for more information.


    .. js:function:: days_in_month()

        Returns the number of days in the month represented by this date

        See the `Rust documentation for days_in_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.days_in_month>`__ for more information.


    .. js:function:: days_in_year()

        Returns the number of days in the year represented by this date

        See the `Rust documentation for days_in_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.days_in_year>`__ for more information.


    .. js:function:: calendar()

        Returns the :js:class:`ICU4XCalendar` object backing this date

        See the `Rust documentation for calendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.calendar>`__ for more information.


.. js:class:: ICU4XIsoDate

    An ICU4X Date object capable of containing a ISO-8601 date

    See the `Rust documentation for Date <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html>`__ for more information.


    .. js:staticfunction:: create(year, month, day)

        Creates a new :js:class:`ICU4XIsoDate` from the specified date and time.

        See the `Rust documentation for try_new_iso_date <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.try_new_iso_date>`__ for more information.


    .. js:function:: to_calendar(calendar)

        Convert this date to one in a different calendar

        See the `Rust documentation for to_calendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.to_calendar>`__ for more information.


    .. js:function:: to_any()

        See the `Rust documentation for to_any <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.to_any>`__ for more information.


    .. js:function:: day_of_month()

        Returns the 1-indexed day in the month for this date

        See the `Rust documentation for day_of_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.day_of_month>`__ for more information.


    .. js:function:: day_of_week()

        Returns the day in the week for this day

        See the `Rust documentation for day_of_week <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.day_of_week>`__ for more information.


    .. js:function:: week_of_month(first_weekday)

        Returns the week number in this month, 1-indexed, based on what is considered the first day of the week (often a locale preference).

        ``first_weekday`` can be obtained via ``first_weekday()`` on :js:class:`ICU4XWeekCalculator`

        See the `Rust documentation for week_of_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.week_of_month>`__ for more information.


    .. js:function:: week_of_year(calculator)

        Returns the week number in this year, using week data

        See the `Rust documentation for week_of_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.week_of_year>`__ for more information.


    .. js:function:: month()

        Returns 1-indexed number of the month of this date in its year

        See the `Rust documentation for month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.month>`__ for more information.


    .. js:function:: year()

        Returns the year number for this date

        See the `Rust documentation for year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.year>`__ for more information.


    .. js:function:: months_in_year()

        Returns the number of months in the year represented by this date

        See the `Rust documentation for months_in_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.months_in_year>`__ for more information.


    .. js:function:: days_in_month()

        Returns the number of days in the month represented by this date

        See the `Rust documentation for days_in_month <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.days_in_month>`__ for more information.


    .. js:function:: days_in_year()

        Returns the number of days in the year represented by this date

        See the `Rust documentation for days_in_year <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.Date.html#method.days_in_year>`__ for more information.


.. js:class:: ICU4XIsoWeekday
