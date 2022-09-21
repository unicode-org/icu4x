``bidi::ffi``
=============

.. js:class:: ICU4XBidi

    An ICU4X Bidi object, containing loaded bidi data

    See the `Rust documentation for BidiClassAdapter <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/bidi/struct.BidiClassAdapter.html>`__ for more information.


    .. js:staticfunction:: create(provider)

        Creates a new :js:class:`ICU4XBidi` from locale data.

        See the `Rust documentation for new <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/bidi/struct.BidiClassAdapter.html#method.new>`__ for more information.


    .. js:function:: for_text(text, default_level)

        Use the data loaded in this object to process a string and calculate bidi information

        Takes in a Level for the default level, if it is an invalid value it will default to LTR

        See the `Rust documentation for new_with_data_source <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.BidiInfo.html#method.new_with_data_source>`__ for more information.


    .. js:staticfunction:: level_is_rtl(level)

        Check if a Level returned by level_at is an RTL level.

        Invalid levels (numbers greater than 125) will be assumed LTR

        See the `Rust documentation for is_rtl <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.is_rtl>`__ for more information.


    .. js:staticfunction:: level_is_ltr(level)

        Check if a Level returned by level_at is an LTR level.

        Invalid levels (numbers greater than 125) will be assumed LTR

        See the `Rust documentation for is_ltr <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.is_ltr>`__ for more information.


    .. js:staticfunction:: level_rtl()

        Get a basic RTL Level value

        See the `Rust documentation for rtl <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.rtl>`__ for more information.


    .. js:staticfunction:: level_ltr()

        Get a simple LTR Level value

        See the `Rust documentation for ltr <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.ltr>`__ for more information.


.. js:class:: ICU4XBidiDirection

.. js:class:: ICU4XBidiInfo

    An object containing bidi information for a given string, produced by ``for_text()`` on ``ICU4XBidi``

    See the `Rust documentation for BidiInfo <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.BidiInfo.html>`__ for more information.


    .. js:function:: paragraph_count()

        The number of paragraphs contained here


    .. js:function:: paragraph_at(n)

        Get the nth paragraph, returning None if out of bounds


    .. js:function:: size()

        The number of bytes in this full text


    .. js:function:: level_at(pos)

        Get the BIDI level at a particular byte index in the full text. This integer is conceptually a ``unicode_bidi::Level``, and can be further inspected using the static methods on ICU4XBidi.

        Returns 0 (equivalent to ``Level::ltr()``) on error


.. js:class:: ICU4XBidiParagraph

    Bidi information for a single processed paragraph


    .. js:function:: set_paragraph_in_text(n)

        Given a paragraph index ``n`` within the surrounding text, this sets this object to the paragraph at that index. Returns ``ICU4XError::OutOfBoundsError`` when out of bounds.

        This is equivalent to calling ``paragraph_at()`` on ``ICU4XBidiInfo`` but doesn't create a new object


    .. js:function:: direction()

        The primary direction of this paragraph

        See the `Rust documentation for level_at <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at>`__ for more information.


    .. js:function:: size()

        The number of bytes in this paragraph

        See the `Rust documentation for len <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.ParagraphInfo.html#method.len>`__ for more information.


    .. js:function:: range_start()

        The start index of this paragraph within the source text


    .. js:function:: range_end()

        The end index of this paragraph within the source text


    .. js:function:: reorder_line(range_start, range_end)

        Reorder a line based on display order. The ranges are specified relative to the source text and must be contained within this paragraph's range.

        See the `Rust documentation for level_at <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at>`__ for more information.


    .. js:function:: level_at(pos)

        Get the BIDI level at a particular byte index in this paragraph. This integer is conceptually a ``unicode_bidi::Level``, and can be further inspected using the static methods on ICU4XBidi.

        Returns 0 (equivalent to ``Level::ltr()``) on error

        See the `Rust documentation for level_at <https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at>`__ for more information.

