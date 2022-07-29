``datetime::ffi``
=================

.. cpp:enum-struct:: ICU4XDateLength

    .. cpp:enumerator:: Full

    .. cpp:enumerator:: Long

    .. cpp:enumerator:: Medium

    .. cpp:enumerator:: Short

.. cpp:class:: ICU4XGregorianDateFormatter

    An ICU4X DateFormatter object capable of formatting a :cpp:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XGregorianDateFormatter, ICU4XError> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XDateLength length)

        Creates a new :cpp:class:`ICU4XGregorianDateFormatter` from locale data.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.DateFormatter.html#method.try_new>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormatter.html#method.format_to_write>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XGregorianDateTime& value) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateFormatter.html#method.format_to_write>`__ for more information.


.. cpp:class:: ICU4XGregorianDateTimeFormatter

    An ICU4X DateFormatter object capable of formatting a :cpp:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XGregorianDateTimeFormatter, ICU4XError> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XHourCyclePreference time_preferences)

        Creates a new :cpp:class:`ICU4XGregorianDateFormatter` from locale data.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html#method.try_new>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html#method.format_to_write>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XGregorianDateTime& value) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.DateTimeFormatter.html#method.format_to_write>`__ for more information.


.. cpp:enum-struct:: ICU4XHourCyclePreference

    .. cpp:enumerator:: H24

    .. cpp:enumerator:: H23

    .. cpp:enumerator:: H12

    .. cpp:enumerator:: H11

    .. cpp:enumerator:: None

.. cpp:class:: ICU4XTimeFormatter

    An ICU4X TimeFormatter object capable of formatting a :cpp:class:`ICU4XGregorianDateTime` as a string

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XTimeFormatter, ICU4XError> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XTimeLength length, ICU4XHourCyclePreference preferences)

        Creates a new :cpp:class:`ICU4XTimeFormatter` from locale data.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.DateFormatter.html#method.try_new>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_gregorian_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html#method.format_to_write>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_gregorian_datetime(const ICU4XGregorianDateTime& value) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html#method.format_to_write>`__ for more information.


.. cpp:enum-struct:: ICU4XTimeLength

    .. cpp:enumerator:: Full

    .. cpp:enumerator:: Long

    .. cpp:enumerator:: Medium

    .. cpp:enumerator:: Short
