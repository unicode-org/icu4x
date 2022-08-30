``time::ffi``
=============

.. cpp:class:: ICU4XTime

    An ICU4X Time object representing a time in terms of hour, minute, second, nanosecond

    See the `Rust documentation for Time <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XTime, ICU4XError> try_new(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond)

        Creates a new :cpp:class:`ICU4XTime` given field values

        See the `Rust documentation for Time <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html>`__ for more information.


    .. cpp:function:: uint8_t hour() const

        Returns the hour in this time

        See the `Rust documentation for hour <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.hour>`__ for more information.


    .. cpp:function:: uint8_t minute() const

        Returns the minute in this time

        See the `Rust documentation for minute <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.minute>`__ for more information.


    .. cpp:function:: uint8_t second() const

        Returns the second in this time

        See the `Rust documentation for second <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.second>`__ for more information.


    .. cpp:function:: uint32_t nanosecond() const

        Returns the nanosecond in this time

        See the `Rust documentation for nanosecond <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/types/struct.Time.html#structfield.nanosecond>`__ for more information.

