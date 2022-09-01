``zoned_formatter::ffi``
========================

.. cpp:class:: ICU4XGregorianZonedDateTimeFormatter

    An object capable of formatting a date time with time zone to a string.

    See the `Rust documentation for ZonedDateTimeFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XGregorianZonedDateTimeFormatter, ICU4XError> try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length)

        Creates a new :cpp:class:`ICU4XGregorianZonedDateTimeFormatter` from locale data.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XGregorianZonedDateTimeFormatter, ICU4XError> try_new_with_iso_8601_time_zone_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XIsoTimeZoneOptions zone_options)

        Creates a new :cpp:class:`ICU4XGregorianZonedDateTimeFormatter` from locale data.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_iso_datetime_with_custom_time_zone_to_writeable(const ICU4XIsoDateTime& datetime, const ICU4XCustomTimeZone& time_zone, W& write) const

        Formats a :cpp:class:`ICU4XIsoDateTime` and :cpp:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.format_to_write>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_iso_datetime_with_custom_time_zone(const ICU4XIsoDateTime& datetime, const ICU4XCustomTimeZone& time_zone) const

        Formats a :cpp:class:`ICU4XIsoDateTime` and :cpp:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.format_to_write>`__ for more information.


.. cpp:class:: ICU4XZonedDateTimeFormatter

    An object capable of formatting a date time with time zone to a string.

    See the `Rust documentation for ZonedDateTimeFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XZonedDateTimeFormatter, ICU4XError> try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length)

        Creates a new :cpp:class:`ICU4XZonedDateTimeFormatter` from locale data.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XZonedDateTimeFormatter, ICU4XError> try_new_with_iso_8601_time_zone_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XIsoTimeZoneOptions zone_options)

        Creates a new :cpp:class:`ICU4XZonedDateTimeFormatter` from locale data.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_datetime_with_custom_time_zone_to_writeable(const ICU4XDateTime& datetime, const ICU4XCustomTimeZone& time_zone, W& write) const

        Formats a :cpp:class:`ICU4XDateTime` and :cpp:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.format_to_write>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_datetime_with_custom_time_zone(const ICU4XDateTime& datetime, const ICU4XCustomTimeZone& time_zone) const

        Formats a :cpp:class:`ICU4XDateTime` and :cpp:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.format_to_write>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_iso_datetime_with_custom_time_zone_to_writeable(const ICU4XIsoDateTime& datetime, const ICU4XCustomTimeZone& time_zone, W& write) const

        Formats a :cpp:class:`ICU4XIsoDateTime` and :cpp:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.format_to_write>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_iso_datetime_with_custom_time_zone(const ICU4XIsoDateTime& datetime, const ICU4XCustomTimeZone& time_zone) const

        Formats a :cpp:class:`ICU4XIsoDateTime` and :cpp:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.format_to_write>`__ for more information.

