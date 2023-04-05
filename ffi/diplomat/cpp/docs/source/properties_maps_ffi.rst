``properties_maps::ffi``
========================

.. cpp:class:: ICU4XCodePointMapData16

    An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.

    For properties whose values fit into 16 bits.

    See the `Rust documentation for properties <https://docs.rs/icu/latest/icu/properties/index.html>`__ for more information.

    See the `Rust documentation for CodePointMapData <https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapData.html>`__ for more information.

    See the `Rust documentation for CodePointMapDataBorrowed <https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html>`__ for more information.


    .. cpp:function:: uint16_t get(char32_t cp) const

        Gets the value for a code point.

        See the `Rust documentation for get <https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get>`__ for more information.


    .. cpp:function:: uint16_t get32(uint32_t cp) const

        Gets the value for a code point (specified as a 32 bit integer, in UTF-32)


    .. cpp:function:: ICU4XCodePointSetData get_set_for_value(uint16_t value) const

        Gets a :cpp:class:`ICU4XCodePointSetData` representing all entries in this map that map to the given value

        See the `Rust documentation for get_set_for_value <https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get_set_for_value>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData16, ICU4XError> load_script(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_script <https://docs.rs/icu/latest/icu/properties/maps/fn.load_script.html>`__ for more information.


.. cpp:class:: ICU4XCodePointMapData8

    An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.

    For properties whose values fit into 8 bits.

    See the `Rust documentation for properties <https://docs.rs/icu/latest/icu/properties/index.html>`__ for more information.

    See the `Rust documentation for CodePointMapData <https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapData.html>`__ for more information.

    See the `Rust documentation for CodePointMapDataBorrowed <https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html>`__ for more information.


    .. cpp:function:: uint8_t get(char32_t cp) const

        Gets the value for a code point.

        See the `Rust documentation for get <https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get>`__ for more information.


    .. cpp:function:: uint8_t get32(uint32_t cp) const

        Gets the value for a code point (specified as a 32 bit integer, in UTF-32)


    .. cpp:function:: ICU4XCodePointSetData get_set_for_value(uint8_t value) const

        Gets a :cpp:class:`ICU4XCodePointSetData` representing all entries in this map that map to the given value

        See the `Rust documentation for get_set_for_value <https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get_set_for_value>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> load_general_category(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_general_category <https://docs.rs/icu/latest/icu/properties/maps/fn.load_general_category.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> load_bidi_class(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_bidi_class <https://docs.rs/icu/latest/icu/properties/maps/fn.load_bidi_class.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> load_east_asian_width(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_east_asian_width <https://docs.rs/icu/latest/icu/properties/maps/fn.load_east_asian_width.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> load_line_break(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_line_break <https://docs.rs/icu/latest/icu/properties/maps/fn.load_line_break.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_grapheme_cluster_break(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_grapheme_cluster_break <https://docs.rs/icu/latest/icu/properties/maps/fn.load_grapheme_cluster_break.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> load_word_break(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_word_break <https://docs.rs/icu/latest/icu/properties/maps/fn.load_word_break.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointMapData8, ICU4XError> load_sentence_break(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_sentence_break <https://docs.rs/icu/latest/icu/properties/maps/fn.load_sentence_break.html>`__ for more information.

