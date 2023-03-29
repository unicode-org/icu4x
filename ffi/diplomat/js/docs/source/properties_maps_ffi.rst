``properties_maps::ffi``
========================

.. js:class:: ICU4XCodePointMapData16

    An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.

    For properties whose values fit into 16 bits.

    See the `Rust documentation for properties <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html>`__ for more information.

    See the `Rust documentation for CodePointMapData <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapData.html>`__ for more information.

    See the `Rust documentation for CodePointMapDataBorrowed <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html>`__ for more information.


    .. js:method:: get(cp)

        Gets the value for a code point.

        See the `Rust documentation for get <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get>`__ for more information.


    .. js:method:: get32(cp)

        Gets the value for a code point (specified as a 32 bit integer, in UTF-32)


    .. js:method:: iter_ranges_for_value(value)

        Produces an iterator over ranges of code points that map to ``value``

        See the `Rust documentation for iter_ranges_for_value <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value>`__ for more information.


    .. js:method:: iter_ranges_for_value_complemented(value)

        Produces an iterator over ranges of code points that do not map to ``value``

        See the `Rust documentation for iter_ranges_for_value_complemented <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value_complemented>`__ for more information.


    .. js:method:: get_set_for_value(value)

        Gets a :js:class:`ICU4XCodePointSetData` representing all entries in this map that map to the given value

        See the `Rust documentation for get_set_for_value <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get_set_for_value>`__ for more information.


    .. js:function:: load_script(provider)

        See the `Rust documentation for load_script <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_script.html>`__ for more information.


.. js:class:: ICU4XCodePointMapData8

    An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.

    For properties whose values fit into 8 bits.

    See the `Rust documentation for properties <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html>`__ for more information.

    See the `Rust documentation for CodePointMapData <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapData.html>`__ for more information.

    See the `Rust documentation for CodePointMapDataBorrowed <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html>`__ for more information.


    .. js:method:: get(cp)

        Gets the value for a code point.

        See the `Rust documentation for get <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get>`__ for more information.


    .. js:method:: get32(cp)

        Gets the value for a code point (specified as a 32 bit integer, in UTF-32)


    .. js:method:: iter_ranges_for_value(value)

        Produces an iterator over ranges of code points that map to ``value``

        See the `Rust documentation for iter_ranges_for_value <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value>`__ for more information.


    .. js:method:: iter_ranges_for_value_complemented(value)

        Produces an iterator over ranges of code points that do not map to ``value``

        See the `Rust documentation for iter_ranges_for_value_complemented <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value_complemented>`__ for more information.


    .. js:method:: get_set_for_value(value)

        Gets a :js:class:`ICU4XCodePointSetData` representing all entries in this map that map to the given value

        See the `Rust documentation for get_set_for_value <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get_set_for_value>`__ for more information.


    .. js:function:: load_general_category(provider)

        See the `Rust documentation for load_general_category <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_general_category.html>`__ for more information.


    .. js:function:: load_bidi_class(provider)

        See the `Rust documentation for load_bidi_class <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_bidi_class.html>`__ for more information.


    .. js:function:: load_east_asian_width(provider)

        See the `Rust documentation for load_east_asian_width <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_east_asian_width.html>`__ for more information.


    .. js:function:: load_line_break(provider)

        See the `Rust documentation for load_line_break <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_line_break.html>`__ for more information.


    .. js:function:: try_grapheme_cluster_break(provider)

        See the `Rust documentation for load_grapheme_cluster_break <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_grapheme_cluster_break.html>`__ for more information.


    .. js:function:: load_word_break(provider)

        See the `Rust documentation for load_word_break <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_word_break.html>`__ for more information.


    .. js:function:: load_sentence_break(provider)

        See the `Rust documentation for load_sentence_break <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_sentence_break.html>`__ for more information.

