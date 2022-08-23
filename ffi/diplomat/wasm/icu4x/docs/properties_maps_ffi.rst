``properties_maps::ffi``
========================

.. js:class:: ICU4XCodePointMapData16

    An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.

    For properties whose values fit into 16 bits.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html>`__ for more information.


    .. js:staticfunction:: try_get_script(provider)

        Gets a map for Unicode property Script from a :js:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_script.html>`__ for more information.


    .. js:function:: get(cp)

        Gets the value for a code point.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get>`__ for more information.


.. js:class:: ICU4XCodePointMapData8

    An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.

    For properties whose values fit into 8 bits.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html>`__ for more information.


    .. js:staticfunction:: try_get_general_category(provider)

        Gets a map for Unicode property General_Category from a :js:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_general_category.html>`__ for more information.


    .. js:staticfunction:: try_get_bidi_class(provider)

        Gets a map for Unicode property Bidi_Class from a :js:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_bidi_class.html>`__ for more information.


    .. js:staticfunction:: try_get_east_asian_width(provider)

        Gets a map for Unicode property East_Asian_Width from a :js:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_east_asian_width.html>`__ for more information.


    .. js:staticfunction:: try_get_line_break(provider)

        Gets a map for Unicode property Line_Break from a :js:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_line_break.html>`__ for more information.


    .. js:staticfunction:: try_grapheme_cluster_break(provider)

        Gets a map for Unicode property Grapheme_Cluster_Break from a :js:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_grapheme_cluster_break.html>`__ for more information.


    .. js:staticfunction:: try_get_word_break(provider)

        Gets a map for Unicode property Word_Break from a :js:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_word_break.html>`__ for more information.


    .. js:staticfunction:: try_get_sentence_break(provider)

        Gets a map for Unicode property Sentence_Break from a :js:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_sentence_break.html>`__ for more information.


    .. js:function:: get(cp)

        Gets the value for a code point.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get>`__ for more information.

