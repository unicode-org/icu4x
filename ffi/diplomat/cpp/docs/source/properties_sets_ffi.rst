``properties_sets::ffi``
========================

.. cpp:class:: ICU4XCodePointSetData

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html>`__ for more information.

    .. cpp:function:: static ICU4XCodePointSetDataResult try_get_ascii_hex_digit(const ICU4XDataProvider& provider)

        Gets a set for Unicode property ascii_hex_digit from a :cpp:class:`ICU4XDataProvider`.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/sets/fn.get_ascii_hex_digit.html>`__ for more information.

    .. cpp:function:: bool contains(char32_t cp) const

        Checks whether the code point is in the set.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_uniset/struct.UnicodeSet.html#method.contains>`__ for more information.

.. cpp:struct:: ICU4XCodePointSetDataResult

    .. cpp:member:: std::optional<ICU4XCodePointSetData> data

        The :cpp:class:`ICU4XCodePointSetData`, if creation was successful.

    .. cpp:member:: bool success

        Whether creating the :cpp:class:`ICU4XCodePointSetData` was successful.
