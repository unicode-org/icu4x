``segmenter_line::ffi``
=======================

.. cpp:class:: ICU4XLineBreakIteratorLatin1

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

.. cpp:class:: ICU4XLineBreakIteratorUtf16

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

.. cpp:class:: ICU4XLineBreakIteratorUtf8

    .. cpp:function:: int32_t next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.

.. cpp:struct:: ICU4XLineBreakOptions

    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.LineBreakOptions.html>`__ for more information.

    .. cpp:member:: ICU4XLineBreakRule line_break_rule

    .. cpp:member:: ICU4XWordBreakRule word_break_rule

    .. cpp:member:: bool ja_zh

.. cpp:enum-struct:: ICU4XLineBreakRule

    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.LineBreakRule.html>`__ for more information.

    .. cpp:enumerator:: Loose

    .. cpp:enumerator:: Normal

    .. cpp:enumerator:: Strict

    .. cpp:enumerator:: Anywhere

.. cpp:class:: ICU4XLineBreakSegmenter

    An ICU4X line-break segmenter, capable of finding breakpoints in strings. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XLineBreakSegmenter, std::monostate> try_new()

        Construct a :cpp:class:`ICU4XLineBreakSegmenter` with default options. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new>`__ for more information.

    .. cpp:function:: static diplomat::result<ICU4XLineBreakSegmenter, std::monostate> try_new_with_options(ICU4XLineBreakOptions options)

        Construct a :cpp:class:`ICU4XLineBreakSegmenter` with custom options. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new_with_options>`__ for more information.

    .. cpp:function:: ICU4XLineBreakIteratorUtf8 segment_utf8(const std::string_view input) const

        Segments a UTF-8 string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_str>`__ for more information.

    .. cpp:function:: ICU4XLineBreakIteratorUtf16 segment_utf16(const diplomat::span<uint16_t> input) const

        Segments a UTF-16 string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_utf16>`__ for more information.

    .. cpp:function:: ICU4XLineBreakIteratorLatin1 segment_latin1(const diplomat::span<uint8_t> input) const

        Segments a Latin-1 string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_latin1>`__ for more information.

.. cpp:enum-struct:: ICU4XWordBreakRule

    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/enum.WordBreakRule.html>`__ for more information.

    .. cpp:enumerator:: Normal

    .. cpp:enumerator:: BreakAll

    .. cpp:enumerator:: KeepAll
