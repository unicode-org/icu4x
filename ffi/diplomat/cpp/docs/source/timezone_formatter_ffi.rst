``timezone_formatter::ffi``
===========================

.. cpp:enum-struct:: ICU4XIsoTimeZoneFormat

    See the `Rust documentation for IsoFormat <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.IsoFormat.html>`__ for more information.


    .. cpp:enumerator:: Basic

    .. cpp:enumerator:: Extended

    .. cpp:enumerator:: UtcBasic

    .. cpp:enumerator:: UtcExtended

.. cpp:enum-struct:: ICU4XIsoTimeZoneMinuteDisplay

    See the `Rust documentation for IsoMinutes <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.IsoMinutes.html>`__ for more information.


    .. cpp:enumerator:: Required

    .. cpp:enumerator:: Optional

.. cpp:enum-struct:: ICU4XIsoTimeZoneSecondDisplay

    See the `Rust documentation for IsoSeconds <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.IsoSeconds.html>`__ for more information.


    .. cpp:enumerator:: Optional

    .. cpp:enumerator:: Never

.. cpp:class:: ICU4XTimeZoneFormatter

    An ICU4X TimeZoneFormatter object capable of formatting an :cpp:class:`ICU4XCustomTimeZone` type (and others) as a string

    See the `Rust documentation for TimeZoneFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XTimeZoneFormatter, ICU4XError> try_new_with_localized_gmt_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale)

        Creates a new :cpp:class:`ICU4XTimeZoneFormatter` from locale data.

        Uses localized GMT as the fallback format.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.try_new_unstable>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.FallbackFormat.html>`__


    .. cpp:function:: static diplomat::result<ICU4XTimeZoneFormatter, ICU4XError> try_new_with_iso_8601_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XIsoTimeZoneFormat format, ICU4XIsoTimeZoneMinuteDisplay minutes, ICU4XIsoTimeZoneSecondDisplay seconds)

        Creates a new :cpp:class:`ICU4XTimeZoneFormatter` from locale data.

        Uses ISO-8601 as the fallback format.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.try_new_unstable>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.FallbackFormat.html>`__


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> load_generic_non_location_long(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_generic_non_location_long <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_generic_non_location_long>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> load_generic_non_location_short(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_generic_non_location_short <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_generic_non_location_short>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> load_specific_non_location_long(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_specific_non_location_long <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_specific_non_location_long>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> load_specific_non_location_short(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_specific_non_location_short <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_specific_non_location_short>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> load_generic_location_format(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_generic_location_format <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_generic_location_format>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> load_localized_gmt_format()

        See the `Rust documentation for load_localized_gmt_format <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_localized_gmt_format>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> load_iso_8601_format(ICU4XIsoTimeZoneFormat format, ICU4XIsoTimeZoneMinuteDisplay minutes, ICU4XIsoTimeZoneSecondDisplay seconds)

        See the `Rust documentation for load_iso_8601_format <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.load_iso_8601_format>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> format_custom_time_zone_to_writeable(const ICU4XCustomTimeZone& value, W& write) const

        Formats a :cpp:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format>`__ for more information.

        See the `Rust documentation for format_to_string <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format_to_string>`__ for more information.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format_to_write>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> format_custom_time_zone(const ICU4XCustomTimeZone& value) const

        Formats a :cpp:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format>`__ for more information.

        See the `Rust documentation for format_to_string <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format_to_string>`__ for more information.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format_to_write>`__ for more information.

