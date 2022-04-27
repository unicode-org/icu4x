``properties_maps::ffi``
========================

.. cpp:class:: ICU4XCodePointMapData16

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. For properties whose values fit into 16 bits.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html>`__ for more information.

    .. cpp:function:: static ICU4XCodePointMapData16Response try_get_script(const ICU4XDataProvider& provider)

        Gets a map for Unicode property Script from a :cpp:class:`ICU4XDataProvider`.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html>`__ for more information.

    .. cpp:function:: uint16_t get(char32_t cp) const

        Gets the value for a code point.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_codepointtrie/codepointtrie/struct.CodePointTrie.html#method.get_u32>`__ for more information.

.. cpp:struct:: ICU4XCodePointMapData16Response

    .. cpp:member:: std::optional<ICU4XCodePointMapData16> data

        The :cpp:class:`ICU4XCodePointMapData16`, if creation was successful.

    .. cpp:member:: bool success

        Whether creating the :cpp:class:`ICU4XCodePointMapData16` was successful.
