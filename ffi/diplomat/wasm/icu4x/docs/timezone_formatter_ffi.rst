``timezone_formatter::ffi``
===========================

.. js:class:: ICU4XIsoTimeZoneFormat

    See the `Rust documentation for IsoFormat <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.IsoFormat.html>`__ for more information.


.. js:class:: ICU4XIsoTimeZoneMinuteDisplay

    See the `Rust documentation for IsoMinutes <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.IsoMinutes.html>`__ for more information.


.. js:class:: ICU4XIsoTimeZoneSecondDisplay

    See the `Rust documentation for IsoSeconds <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.IsoSeconds.html>`__ for more information.


.. js:class:: ICU4XTimeZoneFormatter

    An ICU4X TimeZoneFormatter object capable of formatting an :js:class:`ICU4XCustomTimeZone` type (and others) as a string

    See the `Rust documentation for TimeZoneFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new_with_localized_gmt_fallback(provider, locale)

        Creates a new :js:class:`ICU4XTimeZoneFormatter` from locale data.

        Uses localized GMT as the fallback format.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.try_new_unstable>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.FallbackFormat.html>`__


    .. js:staticfunction:: try_new_with_iso_8601_fallback(provider, locale, format, minutes, seconds)

        Creates a new :js:class:`ICU4XTimeZoneFormatter` from locale data.

        Uses ISO-8601 as the fallback format.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.try_new_unstable>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/time_zone/enum.FallbackFormat.html>`__


    .. js:function:: format_custom_time_zone(value)

        Formats a :js:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TimeZoneFormatter.html#method.format_to_write>`__ for more information.

