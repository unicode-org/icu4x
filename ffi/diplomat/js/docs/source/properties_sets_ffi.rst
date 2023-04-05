``properties_sets::ffi``
========================

.. js:class:: ICU4XCodePointSetData

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.

    See the `Rust documentation for properties <https://docs.rs/icu/latest/icu/properties/index.html>`__ for more information.

    See the `Rust documentation for CodePointSetData <https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetData.html>`__ for more information.

    See the `Rust documentation for CodePointSetDataBorrowed <https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html>`__ for more information.


    .. js:method:: contains(cp)

        Checks whether the code point is in the set.

        See the `Rust documentation for contains <https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.contains>`__ for more information.


    .. js:method:: contains32(cp)

        Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.


    .. js:method:: iter_ranges()

        Produces an iterator over ranges of code points contained in this set

        See the `Rust documentation for iter_ranges <https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.iter_ranges>`__ for more information.


    .. js:method:: iter_ranges_complemented()

        Produces an iterator over ranges of code points not contained in this set

        See the `Rust documentation for iter_ranges_complemented <https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.iter_ranges_complemented>`__ for more information.


    .. js:function:: load_for_general_category_group(provider, group)

        which is a mask with the same format as the ``U_GC_XX_MASK`` mask in ICU4C

        See the `Rust documentation for load_for_general_category_group <https://docs.rs/icu/latest/icu/properties/sets/fn.load_for_general_category_group.html>`__ for more information.


    .. js:function:: load_ascii_hex_digit(provider)

        See the `Rust documentation for load_ascii_hex_digit <https://docs.rs/icu/latest/icu/properties/sets/fn.load_ascii_hex_digit.html>`__ for more information.


    .. js:function:: load_alnum(provider)

        See the `Rust documentation for load_alnum <https://docs.rs/icu/latest/icu/properties/sets/fn.load_alnum.html>`__ for more information.


    .. js:function:: load_alphabetic(provider)

        See the `Rust documentation for load_alphabetic <https://docs.rs/icu/latest/icu/properties/sets/fn.load_alphabetic.html>`__ for more information.


    .. js:function:: load_bidi_control(provider)

        See the `Rust documentation for load_bidi_control <https://docs.rs/icu/latest/icu/properties/sets/fn.load_bidi_control.html>`__ for more information.


    .. js:function:: load_bidi_mirrored(provider)

        See the `Rust documentation for load_bidi_mirrored <https://docs.rs/icu/latest/icu/properties/sets/fn.load_bidi_mirrored.html>`__ for more information.


    .. js:function:: load_blank(provider)

        See the `Rust documentation for load_blank <https://docs.rs/icu/latest/icu/properties/sets/fn.load_blank.html>`__ for more information.


    .. js:function:: load_cased(provider)

        See the `Rust documentation for load_cased <https://docs.rs/icu/latest/icu/properties/sets/fn.load_cased.html>`__ for more information.


    .. js:function:: load_case_ignorable(provider)

        See the `Rust documentation for load_case_ignorable <https://docs.rs/icu/latest/icu/properties/sets/fn.load_case_ignorable.html>`__ for more information.


    .. js:function:: load_full_composition_exclusion(provider)

        See the `Rust documentation for load_full_composition_exclusion <https://docs.rs/icu/latest/icu/properties/sets/fn.load_full_composition_exclusion.html>`__ for more information.


    .. js:function:: load_changes_when_casefolded(provider)

        See the `Rust documentation for load_changes_when_casefolded <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_casefolded.html>`__ for more information.


    .. js:function:: load_changes_when_casemapped(provider)

        See the `Rust documentation for load_changes_when_casemapped <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_casemapped.html>`__ for more information.


    .. js:function:: load_changes_when_nfkc_casefolded(provider)

        See the `Rust documentation for load_changes_when_nfkc_casefolded <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_nfkc_casefolded.html>`__ for more information.


    .. js:function:: load_changes_when_lowercased(provider)

        See the `Rust documentation for load_changes_when_lowercased <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_lowercased.html>`__ for more information.


    .. js:function:: load_changes_when_titlecased(provider)

        See the `Rust documentation for load_changes_when_titlecased <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_titlecased.html>`__ for more information.


    .. js:function:: load_changes_when_uppercased(provider)

        See the `Rust documentation for load_changes_when_uppercased <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_uppercased.html>`__ for more information.


    .. js:function:: load_dash(provider)

        See the `Rust documentation for load_dash <https://docs.rs/icu/latest/icu/properties/sets/fn.load_dash.html>`__ for more information.


    .. js:function:: load_deprecated(provider)

        See the `Rust documentation for load_deprecated <https://docs.rs/icu/latest/icu/properties/sets/fn.load_deprecated.html>`__ for more information.


    .. js:function:: load_default_ignorable_code_point(provider)

        See the `Rust documentation for load_default_ignorable_code_point <https://docs.rs/icu/latest/icu/properties/sets/fn.load_default_ignorable_code_point.html>`__ for more information.


    .. js:function:: load_diacritic(provider)

        See the `Rust documentation for load_diacritic <https://docs.rs/icu/latest/icu/properties/sets/fn.load_diacritic.html>`__ for more information.


    .. js:function:: load_emoji_modifier_base(provider)

        See the `Rust documentation for load_emoji_modifier_base <https://docs.rs/icu/latest/icu/properties/sets/fn.load_emoji_modifier_base.html>`__ for more information.


    .. js:function:: load_emoji_component(provider)

        See the `Rust documentation for load_emoji_component <https://docs.rs/icu/latest/icu/properties/sets/fn.load_emoji_component.html>`__ for more information.


    .. js:function:: load_emoji_modifier(provider)

        See the `Rust documentation for load_emoji_modifier <https://docs.rs/icu/latest/icu/properties/sets/fn.load_emoji_modifier.html>`__ for more information.


    .. js:function:: load_emoji(provider)

        See the `Rust documentation for load_emoji <https://docs.rs/icu/latest/icu/properties/sets/fn.load_emoji.html>`__ for more information.


    .. js:function:: load_emoji_presentation(provider)

        See the `Rust documentation for load_emoji_presentation <https://docs.rs/icu/latest/icu/properties/sets/fn.load_emoji_presentation.html>`__ for more information.


    .. js:function:: load_extender(provider)

        See the `Rust documentation for load_extender <https://docs.rs/icu/latest/icu/properties/sets/fn.load_extender.html>`__ for more information.


    .. js:function:: load_extended_pictographic(provider)

        See the `Rust documentation for load_extended_pictographic <https://docs.rs/icu/latest/icu/properties/sets/fn.load_extended_pictographic.html>`__ for more information.


    .. js:function:: load_graph(provider)

        See the `Rust documentation for load_graph <https://docs.rs/icu/latest/icu/properties/sets/fn.load_graph.html>`__ for more information.


    .. js:function:: load_grapheme_base(provider)

        See the `Rust documentation for load_grapheme_base <https://docs.rs/icu/latest/icu/properties/sets/fn.load_grapheme_base.html>`__ for more information.


    .. js:function:: load_grapheme_extend(provider)

        See the `Rust documentation for load_grapheme_extend <https://docs.rs/icu/latest/icu/properties/sets/fn.load_grapheme_extend.html>`__ for more information.


    .. js:function:: load_grapheme_link(provider)

        See the `Rust documentation for load_grapheme_link <https://docs.rs/icu/latest/icu/properties/sets/fn.load_grapheme_link.html>`__ for more information.


    .. js:function:: load_hex_digit(provider)

        See the `Rust documentation for load_hex_digit <https://docs.rs/icu/latest/icu/properties/sets/fn.load_hex_digit.html>`__ for more information.


    .. js:function:: load_hyphen(provider)

        See the `Rust documentation for load_hyphen <https://docs.rs/icu/latest/icu/properties/sets/fn.load_hyphen.html>`__ for more information.


    .. js:function:: load_id_continue(provider)

        See the `Rust documentation for load_id_continue <https://docs.rs/icu/latest/icu/properties/sets/fn.load_id_continue.html>`__ for more information.


    .. js:function:: load_ideographic(provider)

        See the `Rust documentation for load_ideographic <https://docs.rs/icu/latest/icu/properties/sets/fn.load_ideographic.html>`__ for more information.


    .. js:function:: load_id_start(provider)

        See the `Rust documentation for load_id_start <https://docs.rs/icu/latest/icu/properties/sets/fn.load_id_start.html>`__ for more information.


    .. js:function:: load_ids_binary_operator(provider)

        See the `Rust documentation for load_ids_binary_operator <https://docs.rs/icu/latest/icu/properties/sets/fn.load_ids_binary_operator.html>`__ for more information.


    .. js:function:: load_ids_trinary_operator(provider)

        See the `Rust documentation for load_ids_trinary_operator <https://docs.rs/icu/latest/icu/properties/sets/fn.load_ids_trinary_operator.html>`__ for more information.


    .. js:function:: load_join_control(provider)

        See the `Rust documentation for load_join_control <https://docs.rs/icu/latest/icu/properties/sets/fn.load_join_control.html>`__ for more information.


    .. js:function:: load_logical_order_exception(provider)

        See the `Rust documentation for load_logical_order_exception <https://docs.rs/icu/latest/icu/properties/sets/fn.load_logical_order_exception.html>`__ for more information.


    .. js:function:: load_lowercase(provider)

        See the `Rust documentation for load_lowercase <https://docs.rs/icu/latest/icu/properties/sets/fn.load_lowercase.html>`__ for more information.


    .. js:function:: load_math(provider)

        See the `Rust documentation for load_math <https://docs.rs/icu/latest/icu/properties/sets/fn.load_math.html>`__ for more information.


    .. js:function:: load_noncharacter_code_point(provider)

        See the `Rust documentation for load_noncharacter_code_point <https://docs.rs/icu/latest/icu/properties/sets/fn.load_noncharacter_code_point.html>`__ for more information.


    .. js:function:: load_nfc_inert(provider)

        See the `Rust documentation for load_nfc_inert <https://docs.rs/icu/latest/icu/properties/sets/fn.load_nfc_inert.html>`__ for more information.


    .. js:function:: load_nfd_inert(provider)

        See the `Rust documentation for load_nfd_inert <https://docs.rs/icu/latest/icu/properties/sets/fn.load_nfd_inert.html>`__ for more information.


    .. js:function:: load_nfkc_inert(provider)

        See the `Rust documentation for load_nfkc_inert <https://docs.rs/icu/latest/icu/properties/sets/fn.load_nfkc_inert.html>`__ for more information.


    .. js:function:: load_nfkd_inert(provider)

        See the `Rust documentation for load_nfkd_inert <https://docs.rs/icu/latest/icu/properties/sets/fn.load_nfkd_inert.html>`__ for more information.


    .. js:function:: load_pattern_syntax(provider)

        See the `Rust documentation for load_pattern_syntax <https://docs.rs/icu/latest/icu/properties/sets/fn.load_pattern_syntax.html>`__ for more information.


    .. js:function:: load_pattern_white_space(provider)

        See the `Rust documentation for load_pattern_white_space <https://docs.rs/icu/latest/icu/properties/sets/fn.load_pattern_white_space.html>`__ for more information.


    .. js:function:: load_prepended_concatenation_mark(provider)

        See the `Rust documentation for load_prepended_concatenation_mark <https://docs.rs/icu/latest/icu/properties/sets/fn.load_prepended_concatenation_mark.html>`__ for more information.


    .. js:function:: load_print(provider)

        See the `Rust documentation for load_print <https://docs.rs/icu/latest/icu/properties/sets/fn.load_print.html>`__ for more information.


    .. js:function:: load_quotation_mark(provider)

        See the `Rust documentation for load_quotation_mark <https://docs.rs/icu/latest/icu/properties/sets/fn.load_quotation_mark.html>`__ for more information.


    .. js:function:: load_radical(provider)

        See the `Rust documentation for load_radical <https://docs.rs/icu/latest/icu/properties/sets/fn.load_radical.html>`__ for more information.


    .. js:function:: load_regional_indicator(provider)

        See the `Rust documentation for load_regional_indicator <https://docs.rs/icu/latest/icu/properties/sets/fn.load_regional_indicator.html>`__ for more information.


    .. js:function:: load_soft_dotted(provider)

        See the `Rust documentation for load_soft_dotted <https://docs.rs/icu/latest/icu/properties/sets/fn.load_soft_dotted.html>`__ for more information.


    .. js:function:: load_segment_starter(provider)

        See the `Rust documentation for load_segment_starter <https://docs.rs/icu/latest/icu/properties/sets/fn.load_segment_starter.html>`__ for more information.


    .. js:function:: load_case_sensitive(provider)

        See the `Rust documentation for load_case_sensitive <https://docs.rs/icu/latest/icu/properties/sets/fn.load_case_sensitive.html>`__ for more information.


    .. js:function:: load_sentence_terminal(provider)

        See the `Rust documentation for load_sentence_terminal <https://docs.rs/icu/latest/icu/properties/sets/fn.load_sentence_terminal.html>`__ for more information.


    .. js:function:: load_terminal_punctuation(provider)

        See the `Rust documentation for load_terminal_punctuation <https://docs.rs/icu/latest/icu/properties/sets/fn.load_terminal_punctuation.html>`__ for more information.


    .. js:function:: load_unified_ideograph(provider)

        See the `Rust documentation for load_unified_ideograph <https://docs.rs/icu/latest/icu/properties/sets/fn.load_unified_ideograph.html>`__ for more information.


    .. js:function:: load_uppercase(provider)

        See the `Rust documentation for load_uppercase <https://docs.rs/icu/latest/icu/properties/sets/fn.load_uppercase.html>`__ for more information.


    .. js:function:: load_variation_selector(provider)

        See the `Rust documentation for load_variation_selector <https://docs.rs/icu/latest/icu/properties/sets/fn.load_variation_selector.html>`__ for more information.


    .. js:function:: load_white_space(provider)

        See the `Rust documentation for load_white_space <https://docs.rs/icu/latest/icu/properties/sets/fn.load_white_space.html>`__ for more information.


    .. js:function:: load_xdigit(provider)

        See the `Rust documentation for load_xdigit <https://docs.rs/icu/latest/icu/properties/sets/fn.load_xdigit.html>`__ for more information.


    .. js:function:: load_xid_continue(provider)

        See the `Rust documentation for load_xid_continue <https://docs.rs/icu/latest/icu/properties/sets/fn.load_xid_continue.html>`__ for more information.


    .. js:function:: load_xid_start(provider)

        See the `Rust documentation for load_xid_start <https://docs.rs/icu/latest/icu/properties/sets/fn.load_xid_start.html>`__ for more information.


    .. js:function:: load_for_ecma262(provider, property_name)

        Loads data for a property specified as a string as long as it is one of the `ECMA-262 binary properties <https://tc39.es/ecma262/#table-binary-unicode-properties>`__ (not including Any, ASCII, and Assigned pseudoproperties).

        Returns ``ICU4XError::PropertyUnexpectedPropertyNameError`` in case the string does not match any property in the list

        See the `Rust documentation for load_for_ecma262_unstable <https://docs.rs/icu/latest/icu/properties/sets/fn.load_for_ecma262_unstable.html>`__ for more information.

