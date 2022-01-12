``data_struct::ffi``
====================

.. js:class:: ICU4XDataStruct

    A generic data struct to be used by ICU4X
    This can be used to construct a StructDataProvider.

    .. js:staticfunction:: create_decimal_symbols(plus_sign_prefix, plus_sign_suffix, minus_sign_prefix, minus_sign_suffix, decimal_separator, grouping_separator, primary_group_size, secondary_group_size, min_group_size, digits)

        Construct a new DecimalSymbolsV1 data struct.
        See the `rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/provider/struct.DecimalSymbolsV1.html>`__ for more details.

        - Note: ``digits`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.
