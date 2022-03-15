``segmenter_line::ffi``
=======================

.. js:class:: ICU4XLineBreakIteratorLatin1

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

.. js:class:: ICU4XLineBreakIteratorUtf16

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

.. js:class:: ICU4XLineBreakIteratorUtf8

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

.. js:class:: ICU4XLineBreakOptions

    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.LineBreakOptions.html>`__ for more information.

    .. js:attribute:: line_break_rule

    .. js:attribute:: word_break_rule

    .. js:attribute:: ja_zh

.. js:class:: ICU4XLineBreakRule

    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.LineBreakRule.html>`__ for more information.

.. js:class:: ICU4XLineBreakSegmenter

    An ICU4X line-break segmenter, capable of finding breakpoints in strings. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html>`__ for more information.

    .. js:staticfunction:: try_new(provider)

        Construct a :js:class:`ICU4XLineBreakSegmenter` with default options. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new>`__ for more information.

    .. js:staticfunction:: try_new_with_options(provider, options)

        Construct a :js:class:`ICU4XLineBreakSegmenter` with custom options. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new_with_options>`__ for more information.

    .. js:function:: segment_utf8(input)

        Segments a UTF-8 string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_str>`__ for more information.

    .. js:function:: segment_utf16(input)

        Segments a UTF-16 string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_utf16>`__ for more information.

        - Note: ``input`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.

    .. js:function:: segment_latin1(input)

        Segments a Latin-1 string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_latin1>`__ for more information.

        - Note: ``input`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.

.. js:class:: ICU4XWordBreakRule

    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.WordBreakRule.html>`__ for more information.
