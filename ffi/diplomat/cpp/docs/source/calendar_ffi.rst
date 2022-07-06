``calendar::ffi``
=================

.. cpp:class:: ICU4XGregorianDateTime

    An ICU4X DateTime object capable of containing a Gregorian date and time.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XGregorianDateTime, ICU4XError> new_gregorian_datetime(int32_t year, uint8_t month, uint8_t day, uint8_t hour, uint8_t minute, uint8_t second)

        Creates a new :cpp:class:`ICU4XGregorianDateTime` from locale data.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/struct.DateTime.html#method.new_gregorian_datetime>`__ for more information.
