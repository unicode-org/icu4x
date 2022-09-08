``segmenter_grapheme::ffi``
===========================

.. cpp:class:: ICU4XGraphemeClusterBreakIteratorLatin1

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. cpp:class:: ICU4XGraphemeClusterBreakIteratorUtf16

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. cpp:class:: ICU4XGraphemeClusterBreakIteratorUtf8

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. cpp:class:: ICU4XGraphemeClusterBreakSegmenter

    An ICU4X grapheme-cluster-break segmenter, capable of finding grapheme cluster breakpoints in strings.

    See the `Rust documentation for GraphemeClusterBreakSegmenter <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XGraphemeClusterBreakSegmenter, ICU4XError> try_new(const ICU4XDataProvider& provider)

        Construct an :cpp:class:`ICU4XGraphemeClusterBreakSegmenter`.

        See the `Rust documentation for try_new <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.try_new>`__ for more information.


    .. cpp:function:: ICU4XGraphemeClusterBreakIteratorUtf8 segment_utf8(const std::string_view input) const

        Segments a UTF-8 string.

        See the `Rust documentation for segment_str <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_str>`__ for more information.


        Lifetimes: ``this``, ``input`` must live at least as long as the output.

    .. cpp:function:: ICU4XGraphemeClusterBreakIteratorUtf16 segment_utf16(const diplomat::span<uint16_t> input) const

        Segments a UTF-16 string.

        See the `Rust documentation for segment_utf16 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_utf16>`__ for more information.


        Lifetimes: ``this``, ``input`` must live at least as long as the output.

    .. cpp:function:: ICU4XGraphemeClusterBreakIteratorLatin1 segment_latin1(const diplomat::span<uint8_t> input) const

        Segments a Latin-1 string.

        See the `Rust documentation for segment_latin1 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_latin1>`__ for more information.


        Lifetimes: ``this``, ``input`` must live at least as long as the output.
