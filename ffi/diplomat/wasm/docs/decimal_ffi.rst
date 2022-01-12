``decimal::ffi``
================

.. js:class:: ICU4XFixedDecimalFormat

    An ICU4X Fixed Decimal Format object, capable of formatting a :js:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html>`__ for more information.

    .. js:staticfunction:: try_new(locale, provider, options)

        Creates a new :js:class:`ICU4XFixedDecimalFormat` from locale data. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new>`__ for more information.

    .. js:staticfunction:: try_new_from_decimal_symbols_v1(data_struct, options)

        Creates a new :js:class:`ICU4XFixedDecimalFormat` from preconstructed locale data in the form of an :js:class:`ICU4XDataStruct` constructed from ``ICU4XDataStruct::create_decimal_symbols()``.
        The contents of the data struct will be consumed: if you wish to use the struct again it will have to be reconstructed. Passing a consumed struct to this method will return an error.

    .. js:function:: format(value)

        Formats a :js:class:`ICU4XFixedDecimal` to a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format>`__ for more information.

.. js:class:: ICU4XFixedDecimalFormatOptions

    .. js:attribute:: grouping_strategy

    .. js:attribute:: sign_display

    .. js:staticfunction:: default()

.. js:class:: ICU4XFixedDecimalGroupingStrategy

.. js:class:: ICU4XFixedDecimalSignDisplay
