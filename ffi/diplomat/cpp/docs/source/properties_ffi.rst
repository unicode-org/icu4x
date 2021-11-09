``properties::ffi``
===================

.. cpp:class:: ICU4XUnicodeSetProperty

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html>`__ for more information.

    .. cpp:function:: static ICU4XUnicodeSetPropertyResult try_get_ascii_hex_digit(const ICU4XDataProvider& provider)

    .. cpp:function:: static ICU4XUnicodeSetPropertyResult try_get_ascii_hex_digit_from_static(const ICU4XStaticDataProvider& provider)

    .. cpp:function:: bool contains(char32_t cp) const

.. cpp:struct:: ICU4XUnicodeSetPropertyResult

    .. cpp:member:: std::optional<ICU4XUnicodeSetProperty> data

        The :cpp:class:`ICU4XUnicodeSetProperty`, if creation was successful.

    .. cpp:member:: bool success

        Whether creating the :cpp:class:`ICU4XUnicodeSetProperty` was successful.
