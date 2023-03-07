``properties_sets::ffi``
========================

.. cpp:class:: ICU4XCodePointSetData

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.

    See the `Rust documentation for properties <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html>`__ for more information.

    See the `Rust documentation for CodePointSetData <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetData.html>`__ for more information.

    See the `Rust documentation for CodePointSetDataBorrowed <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetDataBorrowed.html>`__ for more information.


    .. cpp:function:: bool contains(char32_t cp) const

        Checks whether the code point is in the set.

        See the `Rust documentation for contains <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.contains>`__ for more information.


    .. cpp:function:: bool contains32(uint32_t cp) const

        Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_for_general_category_group(const ICU4XDataProvider& provider, uint32_t group)

        which is a mask with the same format as the ``U_GC_XX_MASK`` mask in ICU4C

        See the `Rust documentation for load_for_general_category_group <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_for_general_category_group.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ascii_hex_digit(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_ascii_hex_digit <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ascii_hex_digit.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_alnum(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_alnum <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_alnum.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_alphabetic(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_alphabetic <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_alphabetic.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_bidi_control(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_bidi_control <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_bidi_control.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_bidi_mirrored(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_bidi_mirrored <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_bidi_mirrored.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_blank(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_blank <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_blank.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_cased(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_cased <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_cased.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_case_ignorable(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_case_ignorable <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_case_ignorable.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_full_composition_exclusion(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_full_composition_exclusion <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_full_composition_exclusion.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_casefolded(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_casefolded <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_casefolded.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_casemapped(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_casemapped <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_casemapped.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_nfkc_casefolded(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_nfkc_casefolded <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_nfkc_casefolded.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_lowercased(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_lowercased <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_lowercased.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_titlecased(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_titlecased <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_titlecased.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_uppercased(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_uppercased <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_uppercased.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_dash(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_dash <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_dash.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_deprecated(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_deprecated <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_deprecated.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_default_ignorable_code_point(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_default_ignorable_code_point <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_default_ignorable_code_point.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_diacritic(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_diacritic <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_diacritic.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_modifier_base(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_emoji_modifier_base <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_modifier_base.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_component(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_emoji_component <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_component.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_modifier(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_emoji_modifier <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_modifier.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_emoji <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_presentation(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_emoji_presentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_presentation.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_extender(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_extender <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_extender.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_extended_pictographic(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_extended_pictographic <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_extended_pictographic.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_graph(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_graph <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_graph.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_grapheme_base(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_grapheme_base <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_grapheme_base.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_grapheme_extend(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_grapheme_extend <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_grapheme_extend.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_grapheme_link(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_grapheme_link <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_grapheme_link.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_hex_digit(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_hex_digit <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_hex_digit.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_hyphen(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_hyphen <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_hyphen.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_id_continue(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_id_continue <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_id_continue.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ideographic(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_ideographic <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ideographic.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_id_start(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_id_start <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_id_start.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ids_binary_operator(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_ids_binary_operator <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ids_binary_operator.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ids_trinary_operator(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_ids_trinary_operator <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ids_trinary_operator.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_join_control(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_join_control <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_join_control.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_logical_order_exception(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_logical_order_exception <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_logical_order_exception.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_lowercase(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_lowercase <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_lowercase.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_math(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_math <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_math.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_noncharacter_code_point(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_noncharacter_code_point <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_noncharacter_code_point.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfc_inert(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_nfc_inert <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfc_inert.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfd_inert(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_nfd_inert <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfd_inert.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfkc_inert(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_nfkc_inert <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfkc_inert.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfkd_inert(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_nfkd_inert <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfkd_inert.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_pattern_syntax(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_pattern_syntax <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_pattern_syntax.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_pattern_white_space(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_pattern_white_space <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_pattern_white_space.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_prepended_concatenation_mark(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_prepended_concatenation_mark <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_prepended_concatenation_mark.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_print(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_print <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_print.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_quotation_mark(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_quotation_mark <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_quotation_mark.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_radical(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_radical <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_radical.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_regional_indicator(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_regional_indicator <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_regional_indicator.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_soft_dotted(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_soft_dotted <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_soft_dotted.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_segment_starter(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_segment_starter <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_segment_starter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_case_sensitive(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_case_sensitive <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_case_sensitive.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_sentence_terminal(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_sentence_terminal <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_sentence_terminal.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_terminal_punctuation(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_terminal_punctuation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_terminal_punctuation.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_unified_ideograph(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_unified_ideograph <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_unified_ideograph.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_uppercase(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_uppercase <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_uppercase.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_variation_selector(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_variation_selector <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_variation_selector.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_white_space(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_white_space <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_white_space.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_xdigit(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_xdigit <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_xdigit.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_xid_continue(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_xid_continue <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_xid_continue.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_xid_start(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_xid_start <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_xid_start.html>`__ for more information.


    .. cpp:function:: static diplomat::result<std::optional<ICU4XCodePointSetData>, ICU4XError> load_for_ecma262(const ICU4XDataProvider& provider, const std::string_view property_name)

        See the `Rust documentation for load_for_ecma262_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_for_ecma262_unstable.html>`__ for more information.

