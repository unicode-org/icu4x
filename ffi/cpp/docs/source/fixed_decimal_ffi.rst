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

        Construct an :cpp:class:`ICU4XFixedDecimal` from an integer. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. cpp:function:: static ICU4XCreateFixedDecimalResult create_fromstr(const std::string_view v)

        Construct an :cpp:class:`ICU4XFixedDecimal` from a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. cpp:function:: bool multiply_pow10(int16_t power)

        Multiply the :cpp:class:`ICU4XFixedDecimal` by a given power of ten. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.

    .. cpp:function:: void negate()

        Invert the sign of the :cpp:class:`ICU4XFixedDecimal`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate>`__ for more information.

    .. cpp:function:: template<typename W> void to_string_to_writeable(W& to)

        Format the :cpp:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.

    .. cpp:function:: std::string to_string()

        Format the :cpp:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.
