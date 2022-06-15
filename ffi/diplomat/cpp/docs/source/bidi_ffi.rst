``bidi::ffi``
=============

.. cpp:class:: ICU4XBidi

    An ICU4X Bidi object, containing loaded bidi data
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/bidi/struct.BidiClassAdapter.html>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XBidi, ICU4XError> try_new(const ICU4XDataProvider& provider)

        Creates a new :cpp:class:`ICU4XBidi` from locale data.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/bidi/struct.BidiClassAdapter.html#method.new>`__ for more information.

    .. cpp:function:: ICU4XBidiInfo for_text(const std::string_view text, uint8_t default_level) const

        Use the data loaded in this object to process a string and calculate bidi information
        Takes in a Level for the default level, if it is an invalid value it will default to LTR
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.BidiInfo.html#method.new_with_data_source>`__ for more information.

    .. cpp:function:: static bool level_is_rtl(uint8_t level)

        Check if a Level returned by level_at is an RTL level.
        Invalid levels (numbers greater than 125) will be assumed LTR
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.is_rtl>`__ for more information.

    .. cpp:function:: static bool level_is_ltr(uint8_t level)

        Check if a Level returned by level_at is an LTR level.
        Invalid levels (numbers greater than 125) will be assumed LTR
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.is_ltr>`__ for more information.

    .. cpp:function:: static uint8_t level_rtl()

        Get a basic RTL Level value
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.rtl>`__ for more information.

    .. cpp:function:: static uint8_t level_ltr()

        Get a simple LTR Level value
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.ltr>`__ for more information.

.. cpp:enum-struct:: ICU4XBidiDirection

    .. cpp:enumerator:: Ltr

    .. cpp:enumerator:: Rtl

    .. cpp:enumerator:: Mixed

.. cpp:class:: ICU4XBidiInfo

    An object containing bidi information for a given string, produced by ``for_text()`` on ``ICU4XBidi``
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.BidiInfo.html>`__ for more information.

    .. cpp:function:: size_t paragraph_count() const

        The number of paragraphs contained here

    .. cpp:function:: std::optional<ICU4XBidiParagraph> paragraph_at(size_t n) const

        Get the nth paragraph, returning None if out of bounds

    .. cpp:function:: size_t size() const

        The number of bytes in this full text

    .. cpp:function:: uint8_t level_at(size_t pos) const

        Get the BIDI level at a particular byte index in the full text. This integer is conceptually a ``unicode_bidi::Level``, and can be further inspected using the static methods on ICU4XBidi.
        Returns 0 (equivalent to ``Level::ltr()``) on error

.. cpp:class:: ICU4XBidiParagraph

    Bidi information for a single processed paragraph

    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> set_paragraph_in_text(size_t n)

        Given a paragraph index ``n`` within the surrounding text, this sets this object to the paragraph at that index. Returns ``ICU4XError::OutOfBoundsError`` when out of bounds.
        This is equivalent to calling ``paragraph_at()`` on ``ICU4XBidiInfo`` but doesn't create a new object

    .. cpp:function:: ICU4XBidiDirection direction() const

        The primary direction of this paragraph
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at>`__ for more information.

    .. cpp:function:: size_t size() const

        The number of bytes in this paragraph
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.ParagraphInfo.html#method.len>`__ for more information.

    .. cpp:function:: size_t range_start() const

        The start index of this paragraph within the source text

    .. cpp:function:: size_t range_end() const

        The end index of this paragraph within the source text

    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> reorder_line_to_writeable(size_t range_start, size_t range_end, W& out) const

        Reorder a line based on display order. The ranges are specified relative to the source text and must be contained within this paragraph's range.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at>`__ for more information.

    .. cpp:function:: diplomat::result<std::string, ICU4XError> reorder_line(size_t range_start, size_t range_end) const

        Reorder a line based on display order. The ranges are specified relative to the source text and must be contained within this paragraph's range.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at>`__ for more information.

    .. cpp:function:: uint8_t level_at(size_t pos) const

        Get the BIDI level at a particular byte index in this paragraph. This integer is conceptually a ``unicode_bidi::Level``, and can be further inspected using the static methods on ICU4XBidi.
        Returns 0 (equivalent to ``Level::ltr()``) on error
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at>`__ for more information.
