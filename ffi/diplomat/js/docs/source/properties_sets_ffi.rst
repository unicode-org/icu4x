``properties_sets::ffi``
========================

.. js:class:: ICU4XCodePointSetData

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.

    See the `Rust documentation for properties <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html>`__ for more information.

    See the `Rust documentation for CodePointSetData <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetData.html>`__ for more information.

    See the `Rust documentation for CodePointSetDataBorrowed <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetDataBorrowed.html>`__ for more information.


    .. js:function:: contains(cp)

        Checks whether the code point is in the set.

        See the `Rust documentation for contains <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.contains>`__ for more information.


    .. js:function:: contains32(cp)

        Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.


    .. js:staticfunction:: load_for_general_category_group(provider, group)

        which is a mask with the same format as the ``U_GC_XX_MASK`` mask in ICU4C

        See the `Rust documentation for load_for_general_category_group <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_for_general_category_group.html>`__ for more information.


    .. js:staticfunction:: load_ascii_hex_digit(provider)

        See the `Rust documentation for load_ascii_hex_digit <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ascii_hex_digit.html>`__ for more information.


    .. js:staticfunction:: load_alnum(provider)

        See the `Rust documentation for load_alnum <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_alnum.html>`__ for more information.


    .. js:staticfunction:: load_alphabetic(provider)

        See the `Rust documentation for load_alphabetic <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_alphabetic.html>`__ for more information.


    .. js:staticfunction:: load_bidi_control(provider)

        See the `Rust documentation for load_bidi_control <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_bidi_control.html>`__ for more information.


    .. js:staticfunction:: load_bidi_mirrored(provider)

        See the `Rust documentation for load_bidi_mirrored <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_bidi_mirrored.html>`__ for more information.


    .. js:staticfunction:: load_blank(provider)

        See the `Rust documentation for load_blank <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_blank.html>`__ for more information.


    .. js:staticfunction:: load_cased(provider)

        See the `Rust documentation for load_cased <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_cased.html>`__ for more information.


    .. js:staticfunction:: load_case_ignorable(provider)

        See the `Rust documentation for load_case_ignorable <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_case_ignorable.html>`__ for more information.


    .. js:staticfunction:: load_full_composition_exclusion(provider)

        See the `Rust documentation for load_full_composition_exclusion <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_full_composition_exclusion.html>`__ for more information.


    .. js:staticfunction:: load_changes_when_casefolded(provider)

        See the `Rust documentation for load_changes_when_casefolded <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_casefolded.html>`__ for more information.


    .. js:staticfunction:: load_changes_when_casemapped(provider)

        See the `Rust documentation for load_changes_when_casemapped <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_casemapped.html>`__ for more information.


    .. js:staticfunction:: load_changes_when_nfkc_casefolded(provider)

        See the `Rust documentation for load_changes_when_nfkc_casefolded <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_nfkc_casefolded.html>`__ for more information.


    .. js:staticfunction:: load_changes_when_lowercased(provider)

        See the `Rust documentation for load_changes_when_lowercased <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_lowercased.html>`__ for more information.


    .. js:staticfunction:: load_changes_when_titlecased(provider)

        See the `Rust documentation for load_changes_when_titlecased <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_titlecased.html>`__ for more information.


    .. js:staticfunction:: load_changes_when_uppercased(provider)

        See the `Rust documentation for load_changes_when_uppercased <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_uppercased.html>`__ for more information.


    .. js:staticfunction:: load_dash(provider)

        See the `Rust documentation for load_dash <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_dash.html>`__ for more information.


    .. js:staticfunction:: load_deprecated(provider)

        See the `Rust documentation for load_deprecated <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_deprecated.html>`__ for more information.


    .. js:staticfunction:: load_default_ignorable_code_point(provider)

        See the `Rust documentation for load_default_ignorable_code_point <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_default_ignorable_code_point.html>`__ for more information.


    .. js:staticfunction:: load_diacritic(provider)

        See the `Rust documentation for load_diacritic <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_diacritic.html>`__ for more information.


    .. js:staticfunction:: load_emoji_modifier_base(provider)

        See the `Rust documentation for load_emoji_modifier_base <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_modifier_base.html>`__ for more information.


    .. js:staticfunction:: load_emoji_component(provider)

        See the `Rust documentation for load_emoji_component <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_component.html>`__ for more information.


    .. js:staticfunction:: load_emoji_modifier(provider)

        See the `Rust documentation for load_emoji_modifier <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_modifier.html>`__ for more information.


    .. js:staticfunction:: load_emoji(provider)

        See the `Rust documentation for load_emoji <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji.html>`__ for more information.


    .. js:staticfunction:: load_emoji_presentation(provider)

        See the `Rust documentation for load_emoji_presentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_presentation.html>`__ for more information.


    .. js:staticfunction:: load_extender(provider)

        See the `Rust documentation for load_extender <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_extender.html>`__ for more information.


    .. js:staticfunction:: load_extended_pictographic(provider)

        See the `Rust documentation for load_extended_pictographic <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_extended_pictographic.html>`__ for more information.


    .. js:staticfunction:: load_graph(provider)

        See the `Rust documentation for load_graph <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_graph.html>`__ for more information.


    .. js:staticfunction:: load_grapheme_base(provider)

        See the `Rust documentation for load_grapheme_base <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_grapheme_base.html>`__ for more information.


    .. js:staticfunction:: load_grapheme_extend(provider)

        See the `Rust documentation for load_grapheme_extend <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_grapheme_extend.html>`__ for more information.


    .. js:staticfunction:: load_grapheme_link(provider)

        See the `Rust documentation for load_grapheme_link <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_grapheme_link.html>`__ for more information.


    .. js:staticfunction:: load_hex_digit(provider)

        See the `Rust documentation for load_hex_digit <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_hex_digit.html>`__ for more information.


    .. js:staticfunction:: load_hyphen(provider)

        See the `Rust documentation for load_hyphen <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_hyphen.html>`__ for more information.


    .. js:staticfunction:: load_id_continue(provider)

        See the `Rust documentation for load_id_continue <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_id_continue.html>`__ for more information.


    .. js:staticfunction:: load_ideographic(provider)

        See the `Rust documentation for load_ideographic <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ideographic.html>`__ for more information.


    .. js:staticfunction:: load_id_start(provider)

        See the `Rust documentation for load_id_start <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_id_start.html>`__ for more information.


    .. js:staticfunction:: load_ids_binary_operator(provider)

        See the `Rust documentation for load_ids_binary_operator <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ids_binary_operator.html>`__ for more information.


    .. js:staticfunction:: load_ids_trinary_operator(provider)

        See the `Rust documentation for load_ids_trinary_operator <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ids_trinary_operator.html>`__ for more information.


    .. js:staticfunction:: load_join_control(provider)

        See the `Rust documentation for load_join_control <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_join_control.html>`__ for more information.


    .. js:staticfunction:: load_logical_order_exception(provider)

        See the `Rust documentation for load_logical_order_exception <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_logical_order_exception.html>`__ for more information.


    .. js:staticfunction:: load_lowercase(provider)

        See the `Rust documentation for load_lowercase <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_lowercase.html>`__ for more information.


    .. js:staticfunction:: load_math(provider)

        See the `Rust documentation for load_math <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_math.html>`__ for more information.


    .. js:staticfunction:: load_noncharacter_code_point(provider)

        See the `Rust documentation for load_noncharacter_code_point <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_noncharacter_code_point.html>`__ for more information.


    .. js:staticfunction:: load_nfc_inert(provider)

        See the `Rust documentation for load_nfc_inert <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfc_inert.html>`__ for more information.


    .. js:staticfunction:: load_nfd_inert(provider)

        See the `Rust documentation for load_nfd_inert <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfd_inert.html>`__ for more information.


    .. js:staticfunction:: load_nfkc_inert(provider)

        See the `Rust documentation for load_nfkc_inert <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfkc_inert.html>`__ for more information.


    .. js:staticfunction:: load_nfkd_inert(provider)

        See the `Rust documentation for load_nfkd_inert <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfkd_inert.html>`__ for more information.


    .. js:staticfunction:: load_pattern_syntax(provider)

        See the `Rust documentation for load_pattern_syntax <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_pattern_syntax.html>`__ for more information.


    .. js:staticfunction:: load_pattern_white_space(provider)

        See the `Rust documentation for load_pattern_white_space <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_pattern_white_space.html>`__ for more information.


    .. js:staticfunction:: load_prepended_concatenation_mark(provider)

        See the `Rust documentation for load_prepended_concatenation_mark <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_prepended_concatenation_mark.html>`__ for more information.


    .. js:staticfunction:: load_print(provider)

        See the `Rust documentation for load_print <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_print.html>`__ for more information.


    .. js:staticfunction:: load_quotation_mark(provider)

        See the `Rust documentation for load_quotation_mark <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_quotation_mark.html>`__ for more information.


    .. js:staticfunction:: load_radical(provider)

        See the `Rust documentation for load_radical <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_radical.html>`__ for more information.


    .. js:staticfunction:: load_regional_indicator(provider)

        See the `Rust documentation for load_regional_indicator <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_regional_indicator.html>`__ for more information.


    .. js:staticfunction:: load_soft_dotted(provider)

        See the `Rust documentation for load_soft_dotted <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_soft_dotted.html>`__ for more information.


    .. js:staticfunction:: load_segment_starter(provider)

        See the `Rust documentation for load_segment_starter <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_segment_starter.html>`__ for more information.


    .. js:staticfunction:: load_case_sensitive(provider)

        See the `Rust documentation for load_case_sensitive <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_case_sensitive.html>`__ for more information.


    .. js:staticfunction:: load_sentence_terminal(provider)

        See the `Rust documentation for load_sentence_terminal <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_sentence_terminal.html>`__ for more information.


    .. js:staticfunction:: load_terminal_punctuation(provider)

        See the `Rust documentation for load_terminal_punctuation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_terminal_punctuation.html>`__ for more information.


    .. js:staticfunction:: load_unified_ideograph(provider)

        See the `Rust documentation for load_unified_ideograph <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_unified_ideograph.html>`__ for more information.


    .. js:staticfunction:: load_uppercase(provider)

        See the `Rust documentation for load_uppercase <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_uppercase.html>`__ for more information.


    .. js:staticfunction:: load_variation_selector(provider)

        See the `Rust documentation for load_variation_selector <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_variation_selector.html>`__ for more information.


    .. js:staticfunction:: load_white_space(provider)

        See the `Rust documentation for load_white_space <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_white_space.html>`__ for more information.


    .. js:staticfunction:: load_xdigit(provider)

        See the `Rust documentation for load_xdigit <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_xdigit.html>`__ for more information.


    .. js:staticfunction:: load_xid_continue(provider)

        See the `Rust documentation for load_xid_continue <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_xid_continue.html>`__ for more information.


    .. js:staticfunction:: load_xid_start(provider)

        See the `Rust documentation for load_xid_start <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_xid_start.html>`__ for more information.


    .. js:staticfunction:: load_for_ecma262(provider, property_name)

        Loads data for a property specified as a string as long as it is one of the `ECMA-262 binary properties <https://tc39.es/ecma262/#table-binary-unicode-properties>`__ (not including Any, ASCII, and Assigned pseudoproperties).

        Returns ``ICU4XError::PropertyUnexpectedPropertyNameError`` in case the string does not match any property in the list

        See the `Rust documentation for load_for_ecma262_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_for_ecma262_unstable.html>`__ for more information.

