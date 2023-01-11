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


.. cpp:class:: ICU4XSentenceSegmenter

    An ICU4X sentence-break segmenter, capable of finding sentence breakpoints in strings.

    See the `Rust documentation for SentenceSegmenter <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceSegmenter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XSentenceSegmenter, ICU4XError> create(const ICU4XDataProvider& provider)

        Construct an :cpp:class:`ICU4XSentenceSegmenter`.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceSegmenter.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: ICU4XSentenceBreakIteratorUtf8 segment_utf8(const std::string_view input) const

        Segments a (potentially ill-formed) UTF-8 string.

        See the `Rust documentation for segment_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceSegmenter.html#method.segment_utf8>`__ for more information.

        Lifetimes: ``this``, ``input`` must live at least as long as the output.


    .. cpp:function:: ICU4XSentenceBreakIteratorUtf16 segment_utf16(const diplomat::span<const uint16_t> input) const

        Segments a UTF-16 string.

        See the `Rust documentation for segment_utf16 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceSegmenter.html#method.segment_utf16>`__ for more information.

        Lifetimes: ``this``, ``input`` must live at least as long as the output.


    .. cpp:function:: ICU4XSentenceBreakIteratorLatin1 segment_latin1(const diplomat::span<const uint8_t> input) const

        Segments a Latin-1 string.

        See the `Rust documentation for segment_latin1 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceSegmenter.html#method.segment_latin1>`__ for more information.

        Lifetimes: ``this``, ``input`` must live at least as long as the output.

