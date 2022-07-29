``datetime::ffi``
=================

.. js:class:: ICU4XDateLength

.. js:class:: ICU4XGregorianDateFormatter

    An ICU4X DateFormatter object capable of formatting a :js:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new(locale, provider, length)

        Creates a new :js:class:`ICU4XGregorianDateFormatter` from locale data.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.DateFormatter.html#method.try_new>`__ for more information.


    .. js:function:: format_datetime(value)

        Formats a :js:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormatter.html#method.format_to_write>`__ for more information.


.. js:class:: ICU4XGregorianDateTimeFormatter

    An ICU4X DateFormatter object capable of formatting a :js:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new(locale, provider, date_length, time_length, time_preferences)

        Creates a new :js:class:`ICU4XGregorianDateFormatter` from locale data.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html#method.try_new>`__ for more information.


    .. js:function:: format_datetime(value)

        Formats a :js:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html#method.format_to_write>`__ for more information.


.. js:class:: ICU4XGregorianTimeFormatter

    An ICU4X TimeFormatter object capable of formatting a :js:class:`ICU4XGregorianDateTime` as a string

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new(locale, provider, length, preferences)

        Creates a new :js:class:`ICU4XGregorianTimeFormatter` from locale data.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.DateFormatter.html#method.try_new>`__ for more information.


    .. js:function:: format_datetime(value)

        Formats a :js:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html#method.format_to_write>`__ for more information.


.. js:class:: ICU4XHourCyclePreference

.. js:class:: ICU4XTimeLength
