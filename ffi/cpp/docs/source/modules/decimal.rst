..
    This file is part of ICU4X. For terms of use, please see the file
    called LICENSE at the top level of the ICU4X source tree
    (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

Fixed Decimal Format
====================

This file details the fixed decimal formatting functionality exposed by ``<decimal.hpp>``.

.. cpp:class:: icu4x::FixedDecimalFormat

    An ICU4X Fixed Decimal Format object, capable of formatting a :cpp:class:`FixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html>`_ for more information.

    .. cpp:function:: static std::optional<FixedDecimalFormat> Create(const Locale& locale, const DataProvider& provider, FixedDecimalFormatOptions opts)
 
        Creates a new :cpp:class:`FixedDecimalFormat` from locale data and an options bag. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new>`_ for more information.
 
    .. cpp:function:: std::optional<std::string> Format(const FixedDecimal& dec)
 
        Formats a :cpp:class:`FixedDecimal` to a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format>`_ for more information.

.. cpp:enum-class:: icu4x::GroupingStrategy

    Configuration for how often to render grouping separators. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/options/enum.GroupingStrategy.html>`_ for more information.

    .. cpp:enumerator:: icu4x::GroupingStrategy::Auto
    .. cpp:enumerator:: icu4x::GroupingStrategy::Never
    .. cpp:enumerator:: icu4x::GroupingStrategy::Always
    .. cpp:enumerator:: icu4x::GroupingStrategy::Min2

.. cpp:enum-class:: icu4x::SignDisplay
    
    Configuration for when to render the minus sign or plus sign. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/options/enum.SignDisplay.html>`_ for more information.

    .. cpp:enumerator:: icu4x::SignDisplay::Auto
    .. cpp:enumerator:: icu4x::SignDisplay::Never
    .. cpp:enumerator:: icu4x::SignDisplay::Always
    .. cpp:enumerator:: icu4x::SignDisplay::ExceptZero
    .. cpp:enumerator:: icu4x::SignDisplay::Negative

.. cpp:struct:: icu4x::FixedDecimalFormatOptions
    
    A bag of options defining how numbers will be formatted by :cpp:class:`FixedDecimalFormat`

    .. cpp:member:: GroupingStrategy grouping_strategy
    .. cpp:member:: SignDisplay sign_display
