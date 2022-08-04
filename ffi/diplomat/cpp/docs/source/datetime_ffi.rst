``datetime::ffi``
=================

.. cpp:enum-struct:: ICU4XDateLength

    .. cpp:enumerator:: Full

    .. cpp:enumerator:: Long

    .. cpp:enumerator:: Medium

    .. cpp:enumerator:: Short

.. cpp:class:: ICU4XGregorianDateFormatter

    An ICU4X TypedDateFormatter object capable of formatting a :cpp:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XGregorianDateFormatter, ICU4XError> try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength length)

        Creates a new :cpp:class:`ICU4XGregorianDateFormatter` from locale data.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.TypedDateFormatter.html#method.try_new>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateFormatter.html#method.format_to_write>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XGregorianDateTime& value) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateFormatter.html#method.format_to_write>`__ for more information.


.. cpp:class:: ICU4XGregorianDateTimeFormatter

    An ICU4X TypedDateFormatter object capable of formatting a :cpp:class:`ICU4XGregorianDateTime` as a string, using the Gregorian Calendar.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateTimeFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XGregorianDateTimeFormatter, ICU4XError> try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length)

        Creates a new :cpp:class:`ICU4XGregorianDateFormatter` from locale data.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateTimeFormatter.html#method.try_new>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_datetime_to_writeable(const ICU4XGregorianDateTime& value, W& write) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateTimeFormatter.html#method.format_to_write>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_datetime(const ICU4XGregorianDateTime& value) const

        Formats a :cpp:class:`ICU4XGregorianDateTime` to a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedDateTimeFormatter.html#method.format_to_write>`__ for more information.


.. cpp:class:: ICU4XTimeFormatter

    An ICU4X TimeFormatter object capable of formatting a :cpp:class:`ICU4XGregorianDateTime` as a string

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XTimeFormatter, ICU4XError> try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XTimeLength length)

        Creates a new :cpp:class:`ICU4XTimeFormatter` from locale data.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.TypedDateFormatter.html#method.try_new>`__ for more information.


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
