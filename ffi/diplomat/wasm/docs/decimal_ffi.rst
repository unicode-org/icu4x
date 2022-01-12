``decimal::ffi``
================

.. js:class:: ICU4XFixedDecimalFormat

    An ICU4X Fixed Decimal Format object, capable of formatting a :js:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html>`__ for more information.

    .. js:staticfunction:: try_new(locale, provider, options)

        Creates a new :js:class:`ICU4XFixedDecimalFormat` from locale data. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new>`__ for more information.

    .. js:function:: format(value)

        Formats a :js:class:`ICU4XFixedDecimal` to a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format>`__ for more information.

.. js:class:: ICU4XFixedDecimalFormatOptions

    .. js:attribute:: grouping_strategy

    .. js:attribute:: sign_display

    .. js:staticfunction:: default()

.. js:class:: ICU4XFixedDecimalGroupingStrategy

.. js:class:: ICU4XFixedDecimalSignDisplay
