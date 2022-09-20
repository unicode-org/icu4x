``decimal::ffi``
================

.. js:class:: ICU4XFixedDecimalFormatter

    An ICU4X Fixed Decimal Format object, capable of formatting a :js:class:`ICU4XFixedDecimal` as a string.

    See the `Rust documentation for FixedDecimalFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new_with_grouping_strategy(provider, locale, grouping_strategy)

        Creates a new :js:class:`ICU4XFixedDecimalFormatter` from locale data.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new_unstable>`__ for more information.


    .. js:staticfunction:: try_new_from_decimal_symbols_v1(data_struct, grouping_strategy)

        Creates a new :js:class:`ICU4XFixedDecimalFormatter` from preconstructed locale data in the form of an :js:class:`ICU4XDataStruct` constructed from ``ICU4XDataStruct::create_decimal_symbols()``.

        The contents of the data struct will be consumed: if you wish to use the struct again it will have to be reconstructed. Passing a consumed struct to this method will return an error.


    .. js:function:: format(value)

        Formats a :js:class:`ICU4XFixedDecimal` to a string.

        See the `Rust documentation for format <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormatter.html#method.format>`__ for more information.


.. js:class:: ICU4XFixedDecimalGroupingStrategy

    See the `Rust documentation for GroupingStrategy <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/enum.GroupingStrategy.html>`__ for more information.

