``calendar::ffi``
=================

.. cpp:enum-struct:: ICU4XAnyCalendarKind

    See the `Rust documentation for AnyCalendarKind <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendarKind.html>`__ for more information.


    .. cpp:enumerator:: Iso

        The kind of an Iso calendar


    .. cpp:enumerator:: Gregorian

        The kind of a Gregorian calendar


    .. cpp:enumerator:: Buddhist

        The kind of a Buddhist calendar


    .. cpp:enumerator:: Japanese

        The kind of a Japanese calendar with modern eras


    .. cpp:enumerator:: JapaneseExtended

        The kind of a Japanese calendar with modern and historic eras


    .. cpp:enumerator:: Ethiopian

        The kind of an Ethiopian calendar, with Amete Mihret era


    .. cpp:enumerator:: EthiopianAmeteAlem

        The kind of an Ethiopian calendar, with Amete Alem era


    .. cpp:enumerator:: Indian

        The kind of a Indian calendar


    .. cpp:enumerator:: Coptic

        The kind of a Coptic calendar


.. cpp:class:: ICU4XCalendar

    See the `Rust documentation for AnyCalendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCalendar, ICU4XError> try_new_for_locale(const ICU4XDataProvider& provider, const ICU4XLocale& locale)

        Creates a new :cpp:class:`ICU4XCalendar` from the specified date and time.

        See the `Rust documentation for try_new_for_locale_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.try_new_for_locale_unstable>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCalendar, ICU4XError> try_new_for_kind(const ICU4XDataProvider& provider, ICU4XAnyCalendarKind kind)

        Creates a new :cpp:class:`ICU4XCalendar` from the specified date and time.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: ICU4XAnyCalendarKind kind() const

        Returns the kind of this calendar

        See the `Rust documentation for kind <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.kind>`__ for more information.


.. cpp:class:: ICU4XDateTime

    An ICU4X DateTime object capable of containing a date and time for any calendar.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XDateTime, ICU4XError> try_new_from_iso_in_calendar(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second, const ICU4XCalendar& calendar)

        Creates a new :cpp:class:`ICU4XDateTime` representing the ISO date and time given but in a given calendar

        See the `Rust documentation for new_iso_datetime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_iso_datetime>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> set_ns(uint32_t ns)

        Sets the fractional seconds field of this datetime, in nanoseconds

        See the `Rust documentation for nanosecond <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.nanosecond>`__ for more information.


.. cpp:class:: ICU4XGregorianDateTime

    An ICU4X DateTime object capable of containing a Gregorian date and time.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XGregorianDateTime, ICU4XError> try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second)

        Creates a new :cpp:class:`ICU4XGregorianDateTime` from the specified date and time.

        See the `Rust documentation for new_gregorian_datetime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime>`__ for more information.


.. cpp:class:: ICU4XIsoDateTime

    An ICU4X DateTime object capable of containing a ISO-8601 date and time.

    See the `Rust documentation for DateTime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XIsoDateTime, ICU4XError> try_new(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second)

        Creates a new :cpp:class:`ICU4XIsoDateTime` from the specified date and time.

        See the `Rust documentation for new_gregorian_datetime <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime>`__ for more information.

