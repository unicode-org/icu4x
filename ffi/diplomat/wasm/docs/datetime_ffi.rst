``datetime::ffi``
=================

.. js:class:: ICU4XDateLength

.. js:class:: ICU4XGregorianDateFormat

    An ICU4X DateFormat object capable of formatting a :js:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormat.html>`__ for more information.

    .. js:staticfunction:: try_new(locale, provider, length)

        Creates a new :js:class:`ICU4XGregorianDateFormat` from locale data.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.DateFormat.html#method.try_new>`__ for more information.

    .. js:function:: format_to_write(value)

        Formats a :js:class:`ICU4XGregorianDateTime` to a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormat.html#method.format_to_write>`__ for more information.

.. js:class:: ICU4XGregorianDateTimeFormat

    An ICU4X DateFormat object capable of formatting a :js:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html>`__ for more information.

    .. js:staticfunction:: try_new(locale, provider, date_length, time_length, time_preferences)

        Creates a new :js:class:`ICU4XGregorianDateFormat` from locale data.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html#method.try_new>`__ for more information.

    .. js:function:: format_to_write(value)

        Formats a :js:class:`ICU4XGregorianDateTime` to a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html#method.format_to_write>`__ for more information.

.. js:class:: ICU4XGregorianTimeFormat

    An ICU4X TimeFormat object capable of formatting a :js:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormat.html>`__ for more information.

    .. js:staticfunction:: try_new(locale, provider, length, preferences)

        Creates a new :js:class:`ICU4XGregorianTimeFormat` from locale data.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.DateFormat.html#method.try_new>`__ for more information.

    .. js:function:: format_to_write(value)

        Formats a :js:class:`ICU4XGregorianDateTime` to a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormat.html#method.format_to_write>`__ for more information.

.. js:class:: ICU4XHourCyclePreference

.. js:class:: ICU4XTimeLength
