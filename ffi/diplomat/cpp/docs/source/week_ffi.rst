``week::ffi``
=============

.. cpp:class:: ICU4XWeekCalculator

    A Week calculator, useful to be passed in to ``week_of_year()`` on Date and DateTime types

    See the `Rust documentation for WeekCalculator <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/week/struct.WeekCalculator.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XWeekCalculator, ICU4XError> try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale)

        Creates a new :cpp:class:`ICU4XWeekCalculator` from locale data.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/week/struct.WeekCalculator.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: static ICU4XWeekCalculator new_with_first_day_of_week_and_min_week_days(ICU4XIsoWeekday first_weekday, uint8_t min_week_days)

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/week/struct.WeekCalculator.html#structfield.first_weekday>`__, `2 <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/week/struct.WeekCalculator.html#structfield.min_week_days>`__


    .. cpp:function:: ICU4XIsoWeekday first_weekday() const

        Returns the weekday that starts the week for this object's locale

        See the `Rust documentation for first_weekday <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/week/struct.WeekCalculator.html#structfield.first_weekday>`__ for more information.


    .. cpp:function:: uint8_t min_week_days() const

        The minimum number of days overlapping a year required for a week to be considered part of that year

        See the `Rust documentation for min_week_days <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/week/struct.WeekCalculator.html#structfield.min_week_days>`__ for more information.


.. cpp:struct:: ICU4XWeekOf

    See the `Rust documentation for WeekOf <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/week/struct.WeekOf.html>`__ for more information.


    .. cpp:member:: uint16_t week

    .. cpp:member:: ICU4XWeekRelativeUnit unit

.. cpp:enum-struct:: ICU4XWeekRelativeUnit

    See the `Rust documentation for RelativeUnit <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/week/enum.RelativeUnit.html>`__ for more information.


    .. cpp:enumerator:: Previous

    .. cpp:enumerator:: Current

    .. cpp:enumerator:: Next
