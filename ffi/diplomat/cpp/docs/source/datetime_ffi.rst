``datetime::ffi``
=================

.. cpp:enum-struct:: ICU4XDateLength

    .. cpp:enumerator:: Full

    .. cpp:enumerator:: Long

    .. cpp:enumerator:: Medium

    .. cpp:enumerator:: Short

.. cpp:class:: ICU4XGregorianDateFormat

    An ICU4X DateFormat object capable of formatting a :cpp:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormat.html>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XGregorianDateFormat, ICU4XError> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XDateLength length)

        Creates a new :cpp:class:`ICU4XGregorianDateFormat` from locale data.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.DateFormat.html#method.try_new>`__ for more information.

    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormat.html#method.format_to_write>`__ for more information.

    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XGregorianDateTime& value) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormat.html#method.format_to_write>`__ for more information.

.. cpp:class:: ICU4XGregorianDateTimeFormat

    An ICU4X DateFormat object capable of formatting a :cpp:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XGregorianDateTimeFormat, ICU4XError> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XHourCyclePreference time_preferences)

        Creates a new :cpp:class:`ICU4XGregorianDateFormat` from locale data.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html#method.try_new>`__ for more information.

    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html#method.format_to_write>`__ for more information.

    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XGregorianDateTime& value) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormat.html#method.format_to_write>`__ for more information.

.. cpp:class:: ICU4XGregorianTimeFormat

    An ICU4X TimeFormat object capable of formatting a :cpp:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormat.html>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XGregorianTimeFormat, ICU4XError> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XTimeLength length, ICU4XHourCyclePreference preferences)

        Creates a new :cpp:class:`ICU4XGregorianTimeFormat` from locale data.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.DateFormat.html#method.try_new>`__ for more information.

    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormat.html#method.format_to_write>`__ for more information.

    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XGregorianDateTime& value) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormat.html#method.format_to_write>`__ for more information.

.. cpp:enum-struct:: ICU4XHourCyclePreference

    .. cpp:enumerator:: H24

    .. cpp:enumerator:: H23

    .. cpp:enumerator:: H12

    .. cpp:enumerator:: H11

    .. cpp:enumerator:: None

.. cpp:enum-struct:: ICU4XTimeLength

    .. cpp:enumerator:: Full

    .. cpp:enumerator:: Long

    .. cpp:enumerator:: Medium

    .. cpp:enumerator:: Short
