``calendar::ffi``
=================

.. js:class:: ICU4XAnyCalendarKind

    The various calendar types currently supported by :js:class:`ICU4XCalendar`

    See the `Rust documentation for AnyCalendarKind <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendarKind.html>`__ for more information.


    .. js:staticfunction:: from_locale(locale)

        Read the calendar type off of the -u-ca- extension on a locale

        See the `Rust documentation for from_locale <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendarKind.html#method.from_locale>`__ for more information.


    .. js:staticfunction:: from_bcp47(s)

        Obtain the calendar type given a BCP-47 -u-ca- extension string

        See the `Rust documentation for from_bcp47 <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendarKind.html#method.from_bcp47>`__ for more information.


    .. js:function:: bcp47()

        Obtain the string suitable for use in the -u-ca- extension in a BCP47 locale

        See the `Rust documentation for as_bcp47_string <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendarKind.html#method.as_bcp47_string>`__ for more information.


.. js:class:: ICU4XCalendar

    See the `Rust documentation for AnyCalendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html>`__ for more information.


    .. js:staticfunction:: try_new_for_locale(provider, locale)

        Creates a new :js:class:`ICU4XCalendar` from the specified date and time.

        See the `Rust documentation for try_new_for_locale_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.try_new_for_locale_unstable>`__ for more information.


    .. js:staticfunction:: try_new_for_kind(provider, kind)

        Creates a new :js:class:`ICU4XCalendar` from the specified date and time.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.try_new_unstable>`__ for more information.


    .. js:function:: kind()

        Returns the kind of this calendar

        See the `Rust documentation for kind <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.kind>`__ for more information.

