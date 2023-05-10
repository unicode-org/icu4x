``properties_sets::ffi``
========================

.. cpp:class:: ICU4XCodePointSetData

    An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.

    See the `Rust documentation for properties <https://docs.rs/icu/latest/icu/properties/index.html>`__ for more information.

    See the `Rust documentation for CodePointSetData <https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetData.html>`__ for more information.

    See the `Rust documentation for CodePointSetDataBorrowed <https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html>`__ for more information.


    .. cpp:function:: bool contains(char32_t cp) const

        Checks whether the code point is in the set.

        See the `Rust documentation for contains <https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.contains>`__ for more information.


    .. cpp:function:: bool contains32(uint32_t cp) const

        Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.


    .. cpp:function:: CodePointRangeIterator iter_ranges() const

        Produces an iterator over ranges of code points contained in this set

        See the `Rust documentation for iter_ranges <https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.iter_ranges>`__ for more information.

        Lifetimes: ``this`` must live at least as long as the output.


    .. cpp:function:: CodePointRangeIterator iter_ranges_complemented() const

        Produces an iterator over ranges of code points not contained in this set

        See the `Rust documentation for iter_ranges_complemented <https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.iter_ranges_complemented>`__ for more information.

        Lifetimes: ``this`` must live at least as long as the output.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_for_general_category_group(const ICU4XDataProvider& provider, uint32_t group)

        which is a mask with the same format as the ``U_GC_XX_MASK`` mask in ICU4C

        See the `Rust documentation for load_for_general_category_group <https://docs.rs/icu/latest/icu/properties/sets/fn.load_for_general_category_group.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ascii_hex_digit(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_ascii_hex_digit <https://docs.rs/icu/latest/icu/properties/sets/fn.load_ascii_hex_digit.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_alnum(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_alnum <https://docs.rs/icu/latest/icu/properties/sets/fn.load_alnum.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_alphabetic(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_alphabetic <https://docs.rs/icu/latest/icu/properties/sets/fn.load_alphabetic.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_bidi_control(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_bidi_control <https://docs.rs/icu/latest/icu/properties/sets/fn.load_bidi_control.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_bidi_mirrored(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_bidi_mirrored <https://docs.rs/icu/latest/icu/properties/sets/fn.load_bidi_mirrored.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_blank(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_blank <https://docs.rs/icu/latest/icu/properties/sets/fn.load_blank.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_cased(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_cased <https://docs.rs/icu/latest/icu/properties/sets/fn.load_cased.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_case_ignorable(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_case_ignorable <https://docs.rs/icu/latest/icu/properties/sets/fn.load_case_ignorable.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_full_composition_exclusion(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_full_composition_exclusion <https://docs.rs/icu/latest/icu/properties/sets/fn.load_full_composition_exclusion.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_casefolded(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_casefolded <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_casefolded.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_casemapped(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_casemapped <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_casemapped.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_nfkc_casefolded(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_nfkc_casefolded <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_nfkc_casefolded.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_lowercased(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_lowercased <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_lowercased.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_titlecased(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_titlecased <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_titlecased.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_uppercased(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_changes_when_uppercased <https://docs.rs/icu/latest/icu/properties/sets/fn.load_changes_when_uppercased.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_dash(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_dash <https://docs.rs/icu/latest/icu/properties/sets/fn.load_dash.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_deprecated(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_deprecated <https://docs.rs/icu/latest/icu/properties/sets/fn.load_deprecated.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_default_ignorable_code_point(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_default_ignorable_code_point <https://docs.rs/icu/latest/icu/properties/sets/fn.load_default_ignorable_code_point.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_diacritic(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_diacritic <https://docs.rs/icu/latest/icu/properties/sets/fn.load_diacritic.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_modifier_base(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_emoji_modifier_base <https://docs.rs/icu/latest/icu/properties/sets/fn.load_emoji_modifier_base.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_component(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_emoji_component <https://docs.rs/icu/latest/icu/properties/sets/fn.load_emoji_component.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_modifier(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_emoji_modifier <https://docs.rs/icu/latest/icu/properties/sets/fn.load_emoji_modifier.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_emoji <https://docs.rs/icu/latest/icu/properties/sets/fn.load_emoji.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_presentation(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_emoji_presentation <https://docs.rs/icu/latest/icu/properties/sets/fn.load_emoji_presentation.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_extender(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_extender <https://docs.rs/icu/latest/icu/properties/sets/fn.load_extender.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_extended_pictographic(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_extended_pictographic <https://docs.rs/icu/latest/icu/properties/sets/fn.load_extended_pictographic.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_graph(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_graph <https://docs.rs/icu/latest/icu/properties/sets/fn.load_graph.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_grapheme_base(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_grapheme_base <https://docs.rs/icu/latest/icu/properties/sets/fn.load_grapheme_base.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_grapheme_extend(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_grapheme_extend <https://docs.rs/icu/latest/icu/properties/sets/fn.load_grapheme_extend.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_grapheme_link(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_grapheme_link <https://docs.rs/icu/latest/icu/properties/sets/fn.load_grapheme_link.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_hex_digit(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_hex_digit <https://docs.rs/icu/latest/icu/properties/sets/fn.load_hex_digit.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_hyphen(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_hyphen <https://docs.rs/icu/latest/icu/properties/sets/fn.load_hyphen.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_id_continue(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_id_continue <https://docs.rs/icu/latest/icu/properties/sets/fn.load_id_continue.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ideographic(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_ideographic <https://docs.rs/icu/latest/icu/properties/sets/fn.load_ideographic.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_id_start(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_id_start <https://docs.rs/icu/latest/icu/properties/sets/fn.load_id_start.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ids_binary_operator(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_ids_binary_operator <https://docs.rs/icu/latest/icu/properties/sets/fn.load_ids_binary_operator.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ids_trinary_operator(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_ids_trinary_operator <https://docs.rs/icu/latest/icu/properties/sets/fn.load_ids_trinary_operator.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_join_control(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_join_control <https://docs.rs/icu/latest/icu/properties/sets/fn.load_join_control.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_logical_order_exception(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_logical_order_exception <https://docs.rs/icu/latest/icu/properties/sets/fn.load_logical_order_exception.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_lowercase(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_lowercase <https://docs.rs/icu/latest/icu/properties/sets/fn.load_lowercase.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_math(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_math <https://docs.rs/icu/latest/icu/properties/sets/fn.load_math.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_noncharacter_code_point(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_noncharacter_code_point <https://docs.rs/icu/latest/icu/properties/sets/fn.load_noncharacter_code_point.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfc_inert(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_nfc_inert <https://docs.rs/icu/latest/icu/properties/sets/fn.load_nfc_inert.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfd_inert(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_nfd_inert <https://docs.rs/icu/latest/icu/properties/sets/fn.load_nfd_inert.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfkc_inert(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_nfkc_inert <https://docs.rs/icu/latest/icu/properties/sets/fn.load_nfkc_inert.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfkd_inert(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_nfkd_inert <https://docs.rs/icu/latest/icu/properties/sets/fn.load_nfkd_inert.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_pattern_syntax(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_pattern_syntax <https://docs.rs/icu/latest/icu/properties/sets/fn.load_pattern_syntax.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_pattern_white_space(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_pattern_white_space <https://docs.rs/icu/latest/icu/properties/sets/fn.load_pattern_white_space.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_prepended_concatenation_mark(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_prepended_concatenation_mark <https://docs.rs/icu/latest/icu/properties/sets/fn.load_prepended_concatenation_mark.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_print(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_print <https://docs.rs/icu/latest/icu/properties/sets/fn.load_print.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_quotation_mark(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_quotation_mark <https://docs.rs/icu/latest/icu/properties/sets/fn.load_quotation_mark.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_radical(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_radical <https://docs.rs/icu/latest/icu/properties/sets/fn.load_radical.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_regional_indicator(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_regional_indicator <https://docs.rs/icu/latest/icu/properties/sets/fn.load_regional_indicator.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_soft_dotted(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_soft_dotted <https://docs.rs/icu/latest/icu/properties/sets/fn.load_soft_dotted.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_segment_starter(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_segment_starter <https://docs.rs/icu/latest/icu/properties/sets/fn.load_segment_starter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_case_sensitive(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_case_sensitive <https://docs.rs/icu/latest/icu/properties/sets/fn.load_case_sensitive.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_sentence_terminal(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_sentence_terminal <https://docs.rs/icu/latest/icu/properties/sets/fn.load_sentence_terminal.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_terminal_punctuation(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_terminal_punctuation <https://docs.rs/icu/latest/icu/properties/sets/fn.load_terminal_punctuation.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_unified_ideograph(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_unified_ideograph <https://docs.rs/icu/latest/icu/properties/sets/fn.load_unified_ideograph.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_uppercase(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_uppercase <https://docs.rs/icu/latest/icu/properties/sets/fn.load_uppercase.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_variation_selector(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_variation_selector <https://docs.rs/icu/latest/icu/properties/sets/fn.load_variation_selector.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_white_space(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_white_space <https://docs.rs/icu/latest/icu/properties/sets/fn.load_white_space.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_xdigit(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_xdigit <https://docs.rs/icu/latest/icu/properties/sets/fn.load_xdigit.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_xid_continue(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_xid_continue <https://docs.rs/icu/latest/icu/properties/sets/fn.load_xid_continue.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_xid_start(const ICU4XDataProvider& provider)

        See the `Rust documentation for load_xid_start <https://docs.rs/icu/latest/icu/properties/sets/fn.load_xid_start.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_for_ecma262(const ICU4XDataProvider& provider, const std::string_view property_name)

        Loads data for a property specified as a string as long as it is one of the `ECMA-262 binary properties <https://tc39.es/ecma262/#table-binary-unicode-properties>`__ (not including Any, ASCII, and Assigned pseudoproperties).

        Returns ``ICU4XError::PropertyUnexpectedPropertyNameError`` in case the string does not match any property in the list

        See the `Rust documentation for load_for_ecma262_unstable <https://docs.rs/icu/latest/icu/properties/sets/fn.load_for_ecma262_unstable.html>`__ for more information.

