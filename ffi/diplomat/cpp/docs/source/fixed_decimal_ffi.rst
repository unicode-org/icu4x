``fixed_decimal::ffi``
======================

.. cpp:struct:: ICU4XCreateFixedDecimalResult

    .. cpp:member:: std::optional<ICU4XFixedDecimal> fd

        Will be None if ``success`` is ``false``

    .. cpp:member:: bool success

        Currently just a boolean, but we might add a proper error enum as necessary

.. cpp:class:: ICU4XFixedDecimal

    A decimal number. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. cpp:function:: static ICU4XFixedDecimal create(int32_t v)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an integer.
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. cpp:function:: static std::optional<ICU4XFixedDecimal> create_from_f64(double f)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an float, with enough digits to recover the original floating point in IEEE 754 without needing trailing zeros
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.from_f64>`__ for more information.

    .. cpp:function:: static std::optional<ICU4XFixedDecimal> create_from_f64_with_precision(double f, int16_t precision, ICU4XFixedDecimalRoundingMode rounding_mode)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an float, with a given power of 10 for precision
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.from_f64>`__ for more information.

    .. cpp:function:: static std::optional<ICU4XFixedDecimal> create_from_f64_with_digits(double f, uint8_t digits, ICU4XFixedDecimalRoundingMode rounding_mode)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an float, for a given number of digits
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.from_f64>`__ for more information.

    .. cpp:function:: static ICU4XCreateFixedDecimalResult create_fromstr(const std::string_view v)

        Construct an :cpp:class:`ICU4XFixedDecimal` from a string.
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. cpp:function:: bool multiply_pow10(int16_t power)

        Multiply the :cpp:class:`ICU4XFixedDecimal` by a given power of ten.
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.

    .. cpp:function:: void negate()

        Invert the sign of the :cpp:class:`ICU4XFixedDecimal`.
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate>`__ for more information.

    .. cpp:function:: void pad_or_truncate_left(int16_t shift)

        Add or remove a given number of digits from the left side of the decimal (before the decimal point)
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_or_truncate_left>`__ for more information.

    .. cpp:function:: template<typename W> void to_string_to_writeable(W& to) const

        Format the :cpp:class:`ICU4XFixedDecimal` as a string.
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.

    .. cpp:function:: std::string to_string() const

        Format the :cpp:class:`ICU4XFixedDecimal` as a string.
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.

.. cpp:enum-struct:: ICU4XFixedDecimalRoundingMode

    How to round digits when constructing an ICU4XFixedDecimal from a floating point number

    .. cpp:enumerator:: Truncate

        Truncate leftover digits

    .. cpp:enumerator:: HalfExpand

        Round up from 0.5
