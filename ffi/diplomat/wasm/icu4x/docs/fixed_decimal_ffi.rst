``fixed_decimal::ffi``
======================

.. js:class:: ICU4XFixedDecimal

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. js:staticfunction:: create_from_i64(v)

        Construct an :js:class:`ICU4XFixedDecimal` from an integer.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. js:staticfunction:: create_from_u64(v)

        Construct an :js:class:`ICU4XFixedDecimal` from an integer.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. js:staticfunction:: create_from_f64_with_integer_precision(f)

        Construct an :js:class:`ICU4XFixedDecimal` from an integer-valued float

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. js:staticfunction:: create_from_f64_with_magnitude(f, magnitude)

        Construct an :js:class:`ICU4XFixedDecimal` from an float, with a given power of 10 for the lower magnitude

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. js:staticfunction:: create_from_f64_with_significant_digits(f, digits)

        Construct an :js:class:`ICU4XFixedDecimal` from an float, for a given number of significant digits

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. js:staticfunction:: create_from_f64_with_floating_precision(f)

        Construct an :js:class:`ICU4XFixedDecimal` from an float, with enough digits to recover the original floating point in IEEE 754 without needing trailing zeros

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. js:staticfunction:: create_from_str(v)

        Construct an :js:class:`ICU4XFixedDecimal` from a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. js:function:: digit_at(magnitude)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.digit_at>`__ for more information.


    .. js:function:: magnitude_start()

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.magnitude_range>`__ for more information.


    .. js:function:: magnitude_end()

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.magnitude_range>`__ for more information.


    .. js:function:: nonzero_magnitude_start()

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.nonzero_magnitude_start>`__ for more information.


    .. js:function:: nonzero_magnitude_end()

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.nonzero_magnitude_end>`__ for more information.


    .. js:function:: is_zero()

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.is_zero>`__ for more information.


    .. js:function:: multiply_pow10(power)

        Multiply the :js:class:`ICU4XFixedDecimal` by a given power of ten.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.


    .. js:function:: sign()

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.sign>`__ for more information.


    .. js:function:: set_sign(sign)

        Set the sign of the :js:class:`ICU4XFixedDecimal`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_sign>`__ for more information.


    .. js:function:: apply_sign_display(sign_display)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.apply_sign_display>`__ for more information.


    .. js:function:: trim_start()

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.trim_start>`__ for more information.


    .. js:function:: trim_end()

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.trim_end>`__ for more information.


    .. js:function:: pad_start(position)

        Zero-pad the :js:class:`ICU4XFixedDecimal` on the left to a particular position

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_start>`__ for more information.


    .. js:function:: pad_end(position)

        Zero-pad the :js:class:`ICU4XFixedDecimal` on the right to a particular position

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_end>`__ for more information.


    .. js:function:: set_max_position(position)

        Truncate the :js:class:`ICU4XFixedDecimal` on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years ("2022" -> "22")

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_max_position>`__ for more information.


    .. js:function:: trunc(position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.trunc>`__ for more information.


    .. js:function:: half_trunc(position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_trunc>`__ for more information.


    .. js:function:: expand(position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.expand>`__ for more information.


    .. js:function:: half_expand(position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_expand>`__ for more information.


    .. js:function:: ceil(position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.ceil>`__ for more information.


    .. js:function:: half_ceil(position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_ceil>`__ for more information.


    .. js:function:: floor(position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.floor>`__ for more information.


    .. js:function:: half_floor(position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_floor>`__ for more information.


    .. js:function:: half_even(position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_even>`__ for more information.


    .. js:function:: concatenate_end(other)

        Concatenates ``other`` to the end of ``self``.

        If successful, ``other`` will be set to 0 and a successful status is returned.

        If not successful, ``other`` will be unchanged and an error is returned.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.concatenate_end>`__ for more information.


    .. js:function:: to_string()

        Format the :js:class:`ICU4XFixedDecimal` as a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.


.. js:class:: ICU4XFixedDecimalSign

    The sign of a FixedDecimal, as shown in formatting.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/enum.Sign.html>`__ for more information.


.. js:class:: ICU4XFixedDecimalSignDisplay

    ECMA-402 compatible sign display preference.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/enum.SignDisplay.html>`__ for more information.

