``properties_sets::ffi``
========================

.. js:class:: ICU4XCodePointSetData

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html>`__ for more information.

    .. js:staticfunction:: try_get_ascii_hex_digit(provider)

        Gets a set for Unicode property ascii_hex_digit from a :js:class:`ICU4XDataProvider`.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/sets/fn.get_ascii_hex_digit.html>`__ for more information.

    .. js:function:: contains(cp)

        Checks whether the code point is in the set.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_uniset/struct.UnicodeSet.html#method.contains>`__ for more information.

.. js:class:: ICU4XCodePointSetDataResult

    .. js:attribute:: data

        The :js:class:`ICU4XCodePointSetData`, if creation was successful.

    .. js:attribute:: success

        Whether creating the :js:class:`ICU4XCodePointSetData` was successful.
