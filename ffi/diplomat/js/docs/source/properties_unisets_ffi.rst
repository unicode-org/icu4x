``properties_unisets::ffi``
===========================

.. js:class:: ICU4XUnicodeSetData

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.

    See the `Rust documentation for properties <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html>`__ for more information.

    See the `Rust documentation for UnicodeSetData <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.UnicodeSetData.html>`__ for more information.

    See the `Rust documentation for UnicodeSetDataBorrowed <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.UnicodeSetDataBorrowed.html>`__ for more information.


    .. js:function:: contains(s)

        Checks whether the string is in the set.

        See the `Rust documentation for contains <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.UnicodeSetDataBorrowed.html#method.contains>`__ for more information.


    .. js:function:: contains_char(cp)

        Checks whether the code point is in the set.

        See the `Rust documentation for contains_char <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.UnicodeSetDataBorrowed.html#method.contains_char>`__ for more information.


    .. js:function:: contains32(cp)

        Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.


    .. js:function:: load_basic_emoji(provider)

        See the `Rust documentation for load_basic_emoji <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_basic_emoji.html>`__ for more information.

