``calendar::ffi``
=================

.. js:class:: ICU4XCalendar

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html>`__ for more information.


    .. js:staticfunction:: try_new(provider, locale)

        Creates a new :js:class:`ICU4XGregorianDateTime` from the specified date and time.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.AnyCalendar.html#method.try_new_unstable>`__ for more information.


.. js:class:: ICU4XDateTime

    An ICU4X DateTime object capable of containing a Gregorian date and time.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. js:staticfunction:: try_new_from_iso_in_calendar(year, month, day, hour, minute, second, calendar)

        Creates a new :js:class:`ICU4XDateTime` representing the ISO date and time given but in a given calendar

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_iso_datetime>`__ for more information.


.. js:class:: ICU4XGregorianDateTime

    An ICU4X DateTime object capable of containing a Gregorian date and time.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. js:staticfunction:: try_new(year, month, day, hour, minute, second)

        Creates a new :js:class:`ICU4XGregorianDateTime` from the specified date and time.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime>`__ for more information.

