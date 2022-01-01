``segmenter_line::ffi``
=======================

.. cpp:class:: ICU4XLineBreakIterator

    .. cpp:function:: int32_t next()

        Find the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

.. cpp:class:: ICU4XLineBreakSegmenter

    An ICU4X line-break segmenter, capable of finding breakpoints in strings. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XLineBreakSegmenter, std::monostate> try_new()

        Construct an :cpp:class:`ICU4XLineBreakSegmenter` from an locale identifier. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.from_bytes>`__ for more information.

    .. cpp:function:: ICU4XLineBreakIterator segment_str(const std::string_view input) const
