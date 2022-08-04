``fixed_decimal::ffi``
======================

.. js:class:: ICU4XFixedDecimal

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. js:staticfunction:: create(v)

        Construct an :js:class:`ICU4XFixedDecimal` from an integer.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. js:staticfunction:: create_from_f64_with_max_precision(f)

        Construct an :js:class:`ICU4XFixedDecimal` from an float, with enough digits to recover the original floating point in IEEE 754 without needing trailing zeros

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. js:staticfunction:: create_from_f64_with_lower_magnitude(f, precision)

        Construct an :js:class:`ICU4XFixedDecimal` from an float, with a given power of 10 for the lower magnitude

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. js:staticfunction:: create_from_f64_with_significant_digits(f, digits)

        Construct an :js:class:`ICU4XFixedDecimal` from an float, for a given number of significant digits

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. js:staticfunction:: create_fromstr(v)

        Construct an :js:class:`ICU4XFixedDecimal` from a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. js:function:: multiply_pow10(power)

        Multiply the :js:class:`ICU4XFixedDecimal` by a given power of ten.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.


    .. js:function:: set_sign(sign)

        Set the sign of the :js:class:`ICU4XFixedDecimal`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_sign>`__ for more information.


    .. js:function:: pad_start(position)

        Zero-pad the :js:class:`ICU4XFixedDecimal` on the left to a particular position

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_start>`__ for more information.


    .. js:function:: set_max_position(position)

        Truncate the :js:class:`ICU4XFixedDecimal` on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years ("2022" -> "22")

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_max_position>`__ for more information.


    .. js:function:: pad_end(position)

        Zero-pad the :js:class:`ICU4XFixedDecimal` on the right to a particular position

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_end>`__ for more information.


    .. js:function:: to_string()

        Format the :js:class:`ICU4XFixedDecimal` as a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.


.. js:class:: ICU4XFixedDecimalSign

    The sign of a FixedDecimal, as shown in formatting.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/enum.Sign.html>`__ for more information.

