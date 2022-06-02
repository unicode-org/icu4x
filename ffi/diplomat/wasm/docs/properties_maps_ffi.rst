``properties_maps::ffi``
========================

.. js:class:: ICU4XCodePointMapData16

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property. For properties whose values fit into 16 bits.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/index.html>`__ for more information.

    .. js:staticfunction:: try_get_script(provider)

        Gets a map for Unicode property Script from a :js:class:`ICU4XDataProvider`.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_properties/maps/fn.get_script.html>`__ for more information.

    .. js:function:: get(cp)

        Gets the value for a code point.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_codepointtrie/codepointtrie/struct.CodePointTrie.html#method.get_u32>`__ for more information.

.. js:class:: ICU4XCodePointMapData16Response

    .. js:attribute:: data

        The :js:class:`ICU4XCodePointMapData16`, if creation was successful.

    .. js:attribute:: success

        Whether creating the :js:class:`ICU4XCodePointMapData16` was successful.
