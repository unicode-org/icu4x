``segmenter_sentence::ffi``
===========================

.. cpp:class:: ICU4XSentenceBreakIteratorLatin1

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. cpp:class:: ICU4XSentenceBreakIteratorUtf16

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. cpp:class:: ICU4XSentenceBreakIteratorUtf8

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. cpp:class:: ICU4XSentenceBreakSegmenter

    An ICU4X sentence-break segmenter, capable of finding sentence breakpoints in strings.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XSentenceBreakSegmenter, ICU4XError> try_new(const ICU4XDataProvider& provider)

        Construct an :cpp:class:`ICU4XSentenceBreakSegmenter`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.try_new>`__ for more information.


    .. cpp:function:: ICU4XSentenceBreakIteratorUtf8 segment_utf8(const std::string_view input) const

        Segments a UTF-8 string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.segment_str>`__ for more information.


        Lifetimes: ``this``, ``input`` must live at least as long as the output.

    .. cpp:function:: ICU4XSentenceBreakIteratorUtf16 segment_utf16(const diplomat::span<uint16_t> input) const

        Segments a UTF-16 string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.segment_utf16>`__ for more information.


        Lifetimes: ``this``, ``input`` must live at least as long as the output.

    .. cpp:function:: ICU4XSentenceBreakIteratorLatin1 segment_latin1(const diplomat::span<uint8_t> input) const

        Segments a Latin-1 string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.segment_latin1>`__ for more information.


        Lifetimes: ``this``, ``input`` must live at least as long as the output.
