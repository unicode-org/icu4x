``properties_maps::ffi``
========================

.. js:class:: ICU4XUnicodeScriptMapProperty

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html>`__ for more information.

    .. js:staticfunction:: try_get(provider)

        Gets a set for Unicode property ascii_hex_digit from a :js:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html>`__ for more information.

    .. js:staticfunction:: try_get_from_static(provider)

        Gets a set for Unicode property ascii_hex_digit from a :js:class:`ICU4XStaticDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html>`__ for more information.

    .. js:function:: get(cp)

        Gets the Script for a code point. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_codepointtrie/codepointtrie/struct.CodePointTrie.html#method.get_u32>`__ for more information.

.. js:class:: ICU4XUnicodeScriptMapPropertyResult

    .. js:attribute:: data

        The :js:class:`ICU4XUnicodeScriptMapProperty`, if creation was successful.

    .. js:attribute:: success

        Whether creating the :js:class:`ICU4XUnicodeScriptMapProperty` was successful.
