``fixed_decimal::ffi``
======================

.. cpp:class:: ICU4XFixedDecimal

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. cpp:function:: static ICU4XFixedDecimal create_from_i64(int64_t v)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an integer.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. cpp:function:: static ICU4XFixedDecimal create_from_u64(uint64_t v)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an integer.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_f64_with_integer_precision(double f)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an integer-valued float

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_f64_with_magnitude(double f, int16_t magnitude)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an float, with a given power of 10 for the lower magnitude

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_f64_with_significant_digits(double f, uint8_t digits)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an float, for a given number of significant digits

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_f64_with_floating_precision(double f)

        Construct an :cpp:class:`ICU4XFixedDecimal` from an float, with enough digits to recover the original floating point in IEEE 754 without needing trailing zeros

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.try_from_f64>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XFixedDecimal, ICU4XError> create_from_str(const std::string_view v)

        Construct an :cpp:class:`ICU4XFixedDecimal` from a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.


    .. cpp:function:: uint8_t digit_at(int16_t magnitude) const

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.digit_at>`__ for more information.


    .. cpp:function:: int16_t magnitude_start() const

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.magnitude_range>`__ for more information.


    .. cpp:function:: int16_t magnitude_end() const

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.magnitude_range>`__ for more information.


    .. cpp:function:: int16_t nonzero_magnitude_start() const

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.nonzero_magnitude_start>`__ for more information.


    .. cpp:function:: int16_t nonzero_magnitude_end() const

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.nonzero_magnitude_end>`__ for more information.


    .. cpp:function:: bool is_zero() const

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.is_zero>`__ for more information.


    .. cpp:function:: void multiply_pow10(int16_t power)

        Multiply the :cpp:class:`ICU4XFixedDecimal` by a given power of ten.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.


    .. cpp:function:: ICU4XFixedDecimalSign sign() const

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.sign>`__ for more information.


    .. cpp:function:: void set_sign(ICU4XFixedDecimalSign sign)

        Set the sign of the :cpp:class:`ICU4XFixedDecimal`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_sign>`__ for more information.


    .. cpp:function:: void apply_sign_display(ICU4XFixedDecimalSignDisplay sign_display)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.apply_sign_display>`__ for more information.


    .. cpp:function:: void trim_start()

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.trim_start>`__ for more information.


    .. cpp:function:: void trim_end()

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.trim_end>`__ for more information.


    .. cpp:function:: void pad_start(int16_t position)

        Zero-pad the :cpp:class:`ICU4XFixedDecimal` on the left to a particular position

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_start>`__ for more information.


    .. cpp:function:: void pad_end(int16_t position)

        Zero-pad the :cpp:class:`ICU4XFixedDecimal` on the right to a particular position

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.pad_end>`__ for more information.


    .. cpp:function:: void set_max_position(int16_t position)

        Truncate the :cpp:class:`ICU4XFixedDecimal` on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years ("2022" -> "22")

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.set_max_position>`__ for more information.


    .. cpp:function:: void trunc(int16_t position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.trunc>`__ for more information.


    .. cpp:function:: void half_trunc(int16_t position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_trunc>`__ for more information.


    .. cpp:function:: void expand(int16_t position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.expand>`__ for more information.


    .. cpp:function:: void half_expand(int16_t position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_expand>`__ for more information.


    .. cpp:function:: void ceil(int16_t position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.ceil>`__ for more information.


    .. cpp:function:: void half_ceil(int16_t position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_ceil>`__ for more information.


    .. cpp:function:: void floor(int16_t position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.floor>`__ for more information.


    .. cpp:function:: void half_floor(int16_t position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_floor>`__ for more information.


    .. cpp:function:: void half_even(int16_t position)

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.half_even>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, std::monostate> concatenate_end(ICU4XFixedDecimal& other)

        Concatenates ``other`` to the end of ``self``.

        If successful, ``other`` will be set to 0 and a successful status is returned.

        If not successful, ``other`` will be unchanged and an error is returned.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.concatenate_end>`__ for more information.


    .. cpp:function:: template<typename W> void to_string_to_writeable(W& to) const

        Format the :cpp:class:`ICU4XFixedDecimal` as a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.


    .. cpp:function:: std::string to_string() const

        Format the :cpp:class:`ICU4XFixedDecimal` as a string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.


.. cpp:enum-struct:: ICU4XFixedDecimalSign

    The sign of a FixedDecimal, as shown in formatting.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/enum.Sign.html>`__ for more information.


    .. cpp:enumerator:: None

        No sign (implicitly positive, e.g., 1729).


    .. cpp:enumerator:: Negative

        A negative sign, e.g., -1729.


    .. cpp:enumerator:: Positive

        An explicit positive sign, e.g., +1729.


.. cpp:enum-struct:: ICU4XFixedDecimalSignDisplay

    ECMA-402 compatible sign display preference.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/enum.SignDisplay.html>`__ for more information.


    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: Never

    .. cpp:enumerator:: Always

    .. cpp:enumerator:: ExceptZero

    .. cpp:enumerator:: Negative
