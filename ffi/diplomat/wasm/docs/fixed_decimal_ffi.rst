``fixed_decimal::ffi``
======================

.. js:class:: ICU4XCreateFixedDecimalResult

    .. js:attribute:: fd

        Will be None if ``success`` is ``false``

    .. js:attribute:: success

        Currently just a boolean, but we might add a proper error enum as necessary

.. js:class:: ICU4XFixedDecimal

    A decimal number. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. js:staticfunction:: create(v)

        Construct an :js:class:`ICU4XFixedDecimal` from an integer.
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. js:staticfunction:: create_from_f64_with_max_precision(f)

        Construct an :js:class:`ICU4XFixedDecimal` from an float, with enough digits to recover the original floating point in IEEE 754 without needing trailing zeros
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.from_f64>`__ for more information.

    .. js:staticfunction:: create_from_f64_with_lower_magnitude(f, precision, rounding_mode)

        Construct an :js:class:`ICU4XFixedDecimal` from an float, with a given power of 10 for the lower magnitude
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.from_f64>`__ for more information.

    .. js:staticfunction:: create_from_f64_with_significant_digits(f, digits, rounding_mode)

        Construct an :js:class:`ICU4XFixedDecimal` from an float, for a given number of significant digits
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.from_f64>`__ for more information.

    .. js:staticfunction:: create_fromstr(v)

        Construct an :js:class:`ICU4XFixedDecimal` from a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. js:function:: multiply_pow10(power)

        Multiply the :js:class:`ICU4XFixedDecimal` by a given power of ten. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.

    .. js:function:: negate()

        Invert the sign of the :js:class:`ICU4XFixedDecimal`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate>`__ for more information.

    .. js:function:: pad_left(digits)

        Zero-pad the :js:class:`ICU4XFixedDecimal` on the left to a particular number of integer digits See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_left>`__ for more information.

    .. js:function:: truncate_left(magnitude)

        Truncate the :js:class:`ICU4XFixedDecimal` on the left to a particular magnitude, deleting digits if necessary. This is useful for, e.g. abbreviating years ("2022" -> "22") See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.truncate_left>`__ for more information.

    .. js:function:: pad_right(negative_magnitude)

        Zero-pad the :js:class:`ICU4XFixedDecimal` on the right to a particular (negative) magnitude See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_right>`__ for more information.

    .. js:function:: to_string()

        Format the :js:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.

.. js:class:: ICU4XFixedDecimalRoundingMode

    How to round digits when constructing an ICU4XFixedDecimal from a floating point number
