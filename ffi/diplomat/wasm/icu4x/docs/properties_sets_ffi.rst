``properties_sets::ffi``
========================

.. js:class:: ICU4XCodePointSetData

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.

    See the `Rust documentation for properties <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html>`__ for more information.

    See the `Rust documentation for CodePointSetData <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetData.html>`__ for more information.

    See the `Rust documentation for CodePointSetDataBorrowed <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetDataBorrowed.html>`__ for more information.


    .. js:staticfunction:: try_get_ascii_hex_digit(provider)

        Gets a set for Unicode property ascii_hex_digit from a :js:class:`ICU4XDataProvider`.

        See the `Rust documentation for load_ascii_hex_digit <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ascii_hex_digit.html>`__ for more information.


    .. js:function:: contains(cp)

        Checks whether the code point is in the set.

        See the `Rust documentation for contains <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.contains>`__ for more information.

