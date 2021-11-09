``properties_maps::ffi``
========================

.. cpp:class:: ICU4XUnicodeScriptMapProperty

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html>`__ for more information.

    .. cpp:function:: static ICU4XUnicodeScriptMapPropertyResult try_get(const ICU4XDataProvider& provider)

        Gets a set for Unicode property ascii_hex_digit from a :cpp:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html>`__ for more information.

    .. cpp:function:: static ICU4XUnicodeScriptMapPropertyResult try_get_from_static(const ICU4XStaticDataProvider& provider)

        Gets a set for Unicode property ascii_hex_digit from a :cpp:class:`ICU4XStaticDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html>`__ for more information.

    .. cpp:function:: uint32_t get(char32_t cp) const

        Gets the Script for a code point. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_codepointtrie/codepointtrie/struct.CodePointTrie.html#method.get_u32>`__ for more information.

.. cpp:struct:: ICU4XUnicodeScriptMapPropertyResult

    .. cpp:member:: std::optional<ICU4XUnicodeScriptMapProperty> data

        The :cpp:class:`ICU4XUnicodeScriptMapProperty`, if creation was successful.

    .. cpp:member:: bool success

        Whether creating the :cpp:class:`ICU4XUnicodeScriptMapProperty` was successful.
