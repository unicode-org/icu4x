``datetime::ffi``
=================

.. js:class:: ICU4XDateLength

.. js:class:: ICU4XDateTimeFormatter

    An ICU4X DateFormatter object capable of formatting a :js:class:`ICU4XDateTime` as a string, using some calendar specified at runtime in the locale.

    See the `Rust documentation for DateTimeFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new(provider, locale, date_length, time_length)

        Creates a new :js:class:`ICU4XDateTimeFormatter` from locale data.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html#method.try_new_unstable>`__ for more information.


    .. js:function:: format_datetime(value)

        Formats a :js:class:`ICU4XDateTime` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html#method.format_to_write>`__ for more information.


.. js:class:: ICU4XGregorianDateFormatter

    An ICU4X TypedDateFormatter object capable of formatting a :js:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.

    See the `Rust documentation for TypedDateFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new(provider, locale, length)

        Creates a new :js:class:`ICU4XGregorianDateFormatter` from locale data.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.TypedDateFormatter.html#method.try_new_unstable>`__ for more information.


    .. js:function:: format_datetime(value)

        Formats a :js:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateFormatter.html#method.format_to_write>`__ for more information.


.. js:class:: ICU4XGregorianDateTimeFormatter

    An ICU4X TypedDateFormatter object capable of formatting a :js:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.

    See the `Rust documentation for TypedDateTimeFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateTimeFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new(provider, locale, date_length, time_length)

        Creates a new :js:class:`ICU4XGregorianDateFormatter` from locale data.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateTimeFormatter.html#method.try_new_unstable>`__ for more information.


    .. js:function:: format_datetime(value)

        Formats a :js:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateTimeFormatter.html#method.format_to_write>`__ for more information.


.. js:class:: ICU4XTimeFormatter

    An ICU4X TimeFormatter object capable of formatting a :js:class:`ICU4XGregorianDateTime` as a string

    See the `Rust documentation for TimeFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new(provider, locale, length)

        Creates a new :js:class:`ICU4XTimeFormatter` from locale data.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.TypedDateFormatter.html#method.try_new_unstable>`__ for more information.


    .. js:function:: format_gregorian_datetime(value)

        Formats a :js:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html#method.format_to_write>`__ for more information.


.. js:class:: ICU4XTimeLength
