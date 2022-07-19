``segmenter_word::ffi``
=======================

.. cpp:class:: ICU4XWordBreakIteratorLatin1

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. cpp:class:: ICU4XWordBreakIteratorUtf16

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. cpp:class:: ICU4XWordBreakIteratorUtf8

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. cpp:class:: ICU4XWordBreakSegmenter

    An ICU4X word-break segmenter, capable of finding word breakpoints in strings.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.WordBreakSegmenter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XWordBreakSegmenter, ICU4XError> try_new(const ICU4XDataProvider& provider)

        Construct an :cpp:class:`ICU4XWordBreakSegmenter`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.WordBreakSegmenter.html#method.try_new>`__ for more information.


    .. cpp:function:: ICU4XWordBreakIteratorUtf8 segment_utf8(const std::string_view input) const

        Segments a UTF-8 string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.WordBreakSegmenter.html#method.segment_str>`__ for more information.


        Lifetimes: ``this``, ``input`` must live at least as long as the output.

    .. cpp:function:: ICU4XWordBreakIteratorUtf16 segment_utf16(const diplomat::span<uint16_t> input) const

        Segments a UTF-16 string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.WordBreakSegmenter.html#method.segment_utf16>`__ for more information.


        Lifetimes: ``this``, ``input`` must live at least as long as the output.

    .. cpp:function:: ICU4XWordBreakIteratorLatin1 segment_latin1(const diplomat::span<uint8_t> input) const

        Segments a Latin-1 string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.WordBreakSegmenter.html#method.segment_latin1>`__ for more information.


        Lifetimes: ``this``, ``input`` must live at least as long as the output.
