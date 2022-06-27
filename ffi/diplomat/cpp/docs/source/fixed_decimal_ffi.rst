``fixed_decimal::ffi``
======================

.. cpp:class:: ICU4XFixedDecimal

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. cpp:function:: static ICU4XFixedDecimal create(int32_t v)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an integer.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_f64_with_max_precision(double f)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an float, with enough digits to recover the original floating point in IEEE 754 without needing trailing zeros
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_f64_with_lower_magnitude(double f, int16_t precision)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an float, with a given power of 10 for the lower magnitude
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_f64_with_significant_digits(double f, uint8_t digits)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an float, for a given number of significant digits
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_fromstr(const std::string_view v)

        Construct an :cpp:class:`ICU4XFixedDecimal` from a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. cpp:function:: bool multiply_pow10(int16_t power)

        Multiply the :cpp:class:`ICU4XFixedDecimal` by a given power of ten.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.

    .. cpp:function:: void set_sign(ICU4XFixedDecimalSign sign)

        Set the sign of the :cpp:class:`ICU4XFixedDecimal`.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate>`__ for more information.

    .. cpp:function:: void pad_left(int16_t position)

        Zero-pad the :cpp:class:`ICU4XFixedDecimal` on the left to a particular position
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_left>`__ for more information.

    .. cpp:function:: void truncate_left(int16_t position)

        Truncate the :cpp:class:`ICU4XFixedDecimal` on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years ("2022" -> "22")
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.truncate_left>`__ for more information.

    .. cpp:function:: void pad_right(int16_t position)

        Zero-pad the :cpp:class:`ICU4XFixedDecimal` on the right to a particular position
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_right>`__ for more information.

    .. cpp:function:: template<typename W> void to_string_to_writeable(W& to) const

        Format the :cpp:class:`ICU4XFixedDecimal` as a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.

    .. cpp:function:: std::string to_string() const

        Format the :cpp:class:`ICU4XFixedDecimal` as a string.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.

.. cpp:enum-struct:: ICU4XFixedDecimalSign

    The sign of a FixedDecimal, as shown in formatting.

    .. cpp:enumerator:: None

        No sign (implicitly positive, e.g., 1729).

    .. cpp:enumerator:: Negative

        A negative sign, e.g., -1729.

    .. cpp:enumerator:: Positive

        An explicit positive sign, e.g., +1729.
