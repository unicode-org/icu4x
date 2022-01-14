``decimal::ffi``
================

.. cpp:class:: ICU4XFixedDecimalFormat

    An ICU4X Fixed Decimal Format object, capable of formatting a :cpp:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XFixedDecimalFormat, std::monostate> try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options)

        Creates a new :cpp:class:`ICU4XFixedDecimalFormat` from locale data. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XFixedDecimalFormat, std::monostate> try_new_from_decimal_symbols_v1(const ICU4XDataStruct& data_struct, ICU4XFixedDecimalFormatOptions options)

        Creates a new :cpp:class:`ICU4XFixedDecimalFormat` from preconstructed locale data in the form of an :cpp:class:`ICU4XDataStruct` constructed from ``ICU4XDataStruct::create_decimal_symbols()``.
        The contents of the data struct will be consumed: if you wish to use the struct again it will have to be reconstructed. Passing a consumed struct to this method will return an error.

    .. cpp:function:: template<typename W> diplomat::result<std::monostate, std::monostate> format_to_writeable(const ICU4XFixedDecimal& value, W& write) const

        Formats a :cpp:class:`ICU4XFixedDecimal` to a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format>`__ for more information.

    .. cpp:function:: diplomat::result<std::string, std::monostate> format(const ICU4XFixedDecimal& value) const

        Formats a :cpp:class:`ICU4XFixedDecimal` to a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format>`__ for more information.

.. cpp:struct:: ICU4XFixedDecimalFormatOptions

    .. cpp:member:: ICU4XFixedDecimalGroupingStrategy grouping_strategy

    .. cpp:member:: ICU4XFixedDecimalSignDisplay sign_display

    .. cpp:function:: static ICU4XFixedDecimalFormatOptions default_()

.. cpp:enum-struct:: ICU4XFixedDecimalGroupingStrategy

    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: Never

    .. cpp:enumerator:: Always

    .. cpp:enumerator:: Min2

.. cpp:enum-struct:: ICU4XFixedDecimalSignDisplay

    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: Never

    .. cpp:enumerator:: Always

    .. cpp:enumerator:: ExceptZero

    .. cpp:enumerator:: Negative
