``segmenter_line::ffi``
=======================

.. js:class:: ICU4XLineBreakIterator

    .. js:function:: next()

        Find the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

.. js:class:: ICU4XLineBreakSegmenter

    An ICU4X line-break segmenter, capable of finding breakpoints in strings. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html>`__ for more information.

    .. js:staticfunction:: try_new()

        Construct an :js:class:`ICU4XLineBreakSegmenter` from an locale identifier. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.from_bytes>`__ for more information.

    .. js:function:: segment_str(input)
