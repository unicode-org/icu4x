``properties_sets::ffi``
========================

.. cpp:class:: ICU4XUnicodeSetProperty

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html>`__ for more information.

    .. cpp:function:: static ICU4XUnicodeSetPropertyResult try_get_ascii_hex_digit(const ICU4XDataProvider& provider)

        Gets a set for Unicode property ascii_hex_digit from a :cpp:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/sets/fn.get_ascii_hex_digit.html>`__ for more information.

    .. cpp:function:: static ICU4XUnicodeSetPropertyResult try_get_ascii_hex_digit_from_static(const ICU4XStaticDataProvider& provider)

        Gets a set for Unicode property ascii_hex_digit from a :cpp:class:`ICU4XStaticDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/sets/fn.get_ascii_hex_digit.html>`__ for more information.

    .. cpp:function:: bool contains(char32_t cp) const

        Checks whether the code point is in the set. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_uniset/struct.UnicodeSet.html#method.contains>`__ for more information.

.. cpp:struct:: ICU4XUnicodeSetPropertyResult

    .. cpp:member:: std::optional<ICU4XUnicodeSetProperty> data

        The :cpp:class:`ICU4XUnicodeSetProperty`, if creation was successful.

    .. cpp:member:: bool success

        Whether creating the :cpp:class:`ICU4XUnicodeSetProperty` was successful.
