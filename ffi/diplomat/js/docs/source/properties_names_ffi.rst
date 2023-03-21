``properties_names::ffi``
=========================

.. js:class:: ICU4XGeneralCategoryNameToMaskMapper

    A type capable of looking up General Category mask values from a string name.

    See the `Rust documentation for get_name_to_enum_mapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/struct.GeneralCategoryGroup.html#method.get_name_to_enum_mapper>`__ for more information.

    See the `Rust documentation for PropertyValueNameToEnumMapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/names/struct.PropertyValueNameToEnumMapper.html>`__ for more information.


    .. js:function:: get_strict(name)

        Get the mask value matching the given name, using strict matching

        Returns 0 if the name is unknown for this property


    .. js:function:: get_loose(name)

        Get the mask value matching the given name, using loose matching

        Returns 0 if the name is unknown for this property


    .. js:staticfunction:: load(provider)

        See the `Rust documentation for get_name_to_enum_mapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/struct.GeneralCategoryGroup.html#method.get_name_to_enum_mapper>`__ for more information.


.. js:class:: ICU4XPropertyValueNameToEnumMapper

    A type capable of looking up a property value from a string name.

    See the `Rust documentation for PropertyValueNameToEnumMapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/names/struct.PropertyValueNameToEnumMapper.html>`__ for more information.

    See the `Rust documentation for PropertyValueNameToEnumMapperBorrowed <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/names/struct.PropertyValueNameToEnumMapperBorrowed.html>`__ for more information.


    .. js:function:: get_strict(name)

        Get the property value matching the given name, using strict matching

        Returns -1 if the name is unknown for this property

        See the `Rust documentation for get_strict <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/names/struct.PropertyValueNameToEnumMapperBorrowed.html#method.get_strict>`__ for more information.


    .. js:function:: get_loose(name)

        Get the property value matching the given name, using loose matching

        Returns -1 if the name is unknown for this property

        See the `Rust documentation for get_loose <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/names/struct.PropertyValueNameToEnumMapperBorrowed.html#method.get_loose>`__ for more information.


    .. js:staticfunction:: load_general_category(provider)

        See the `Rust documentation for get_name_to_enum_mapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/struct.GeneralCategory.html#method.get_name_to_enum_mapper>`__ for more information.


    .. js:staticfunction:: load_bidi_class(provider)

        See the `Rust documentation for get_name_to_enum_mapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/struct.BidiClass.html#method.get_name_to_enum_mapper>`__ for more information.


    .. js:staticfunction:: load_east_asian_width(provider)

        See the `Rust documentation for get_name_to_enum_mapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/struct.EastAsianWidth.html#method.get_name_to_enum_mapper>`__ for more information.


    .. js:staticfunction:: load_line_break(provider)

        See the `Rust documentation for get_name_to_enum_mapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/struct.LineBreak.html#method.get_name_to_enum_mapper>`__ for more information.


    .. js:staticfunction:: load_grapheme_cluster_break(provider)

        See the `Rust documentation for get_name_to_enum_mapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/struct.GraphemeClusterBreak.html#method.get_name_to_enum_mapper>`__ for more information.


    .. js:staticfunction:: load_word_break(provider)

        See the `Rust documentation for get_name_to_enum_mapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/struct.WordBreak.html#method.get_name_to_enum_mapper>`__ for more information.


    .. js:staticfunction:: load_sentence_break(provider)

        See the `Rust documentation for get_name_to_enum_mapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/struct.SentenceBreak.html#method.get_name_to_enum_mapper>`__ for more information.


    .. js:staticfunction:: load_script(provider)

        See the `Rust documentation for get_name_to_enum_mapper <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/struct.Script.html#method.get_name_to_enum_mapper>`__ for more information.

