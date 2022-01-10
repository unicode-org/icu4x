``segmenter_line::ffi``
=======================

.. js:class:: ICU4XLineBreakIterator

    .. js:function:: next()

        Find the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

.. js:class:: ICU4XLineBreakOptions

    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.LineBreakOptions.html>`__ for more information.

    .. js:attribute:: line_break_rule

    .. js:attribute:: word_break_rule

    .. js:attribute:: ja_zh

.. js:class:: ICU4XLineBreakRule

    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.LineBreakRule.html>`__ for more information.

.. js:class:: ICU4XLineBreakSegmenter

    An ICU4X line-break segmenter, capable of finding breakpoints in strings. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html>`__ for more information.

    .. js:staticfunction:: try_new()

        Construct a :js:class:`ICU4XLineBreakSegmenter` with default options. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new>`__ for more information.

    .. js:staticfunction:: try_new_with_options(options)

        Construct a :js:class:`ICU4XLineBreakSegmenter` with custom options. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new_with_options>`__ for more information.

    .. js:function:: segment_str(input)

.. js:class:: ICU4XWordBreakRule

    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.WordBreakRule.html>`__ for more information.
