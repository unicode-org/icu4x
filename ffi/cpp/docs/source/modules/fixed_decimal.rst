..
    This file is part of ICU4X. For terms of use, please see the file
    called LICENSE at the top level of the ICU4X source tree
    (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

Fixed Decimal
=============

This file details the locale-handling utilities exposed by ``<fixed_decimal.hpp>``.

.. cpp:class:: icu4x::FixedDecimal

    A decimal number. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`_ for more information.
    .. cpp:function:: FixedDecimal::FixedDecimal(int64_t number)

        Construct a :cpp:class:`FixedDecimal` from an integer.

    .. cpp:function:: void FixedDecimal::MultiplyPow10(int16_t power)

        Multiply the :cpp:class:`FixedDecimal` by a given power of ten. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`_ for more information.
    .. cpp:function:: void FixedDecimal::Negate()

        Invert the sign of the :cpp:class:`FixedDecimal`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate>`_ for more information.
