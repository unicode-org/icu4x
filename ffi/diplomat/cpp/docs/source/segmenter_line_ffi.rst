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


.. cpp:struct:: ICU4XLineBreakOptionsV1

    See the `Rust documentation for LineBreakOptions <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineBreakOptions.html>`__ for more information.


    .. cpp:member:: ICU4XLineBreakRule line_break_rule

    .. cpp:member:: ICU4XWordBreakRule word_break_rule

    .. cpp:member:: bool ja_zh

.. cpp:enum-struct:: ICU4XLineBreakRule

    See the `Rust documentation for LineBreakRule <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/enum.LineBreakRule.html>`__ for more information.


    .. cpp:enumerator:: Loose

    .. cpp:enumerator:: Normal

    .. cpp:enumerator:: Strict

    .. cpp:enumerator:: Anywhere

.. cpp:class:: ICU4XLineSegmenter

    An ICU4X line-break segmenter, capable of finding breakpoints in strings.

    See the `Rust documentation for LineSegmenter <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XLineSegmenter, ICU4XError> create(const ICU4XDataProvider& provider)

        Construct a :cpp:class:`ICU4XLineSegmenter` with default options.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XLineSegmenter, ICU4XError> create_with_options_v1(const ICU4XDataProvider& provider, ICU4XLineBreakOptionsV1 options)

        Construct a :cpp:class:`ICU4XLineSegmenter` with custom options.

        See the `Rust documentation for try_new_with_options_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.try_new_with_options_unstable>`__ for more information.


    .. cpp:function:: ICU4XLineBreakIteratorUtf8 segment_utf8(const std::string_view input) const

        Segments a (potentially ill-formed) UTF-8 string.

        See the `Rust documentation for segment_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.segment_utf8>`__ for more information.

        Lifetimes: ``this``, ``input`` must live at least as long as the output.


    .. cpp:function:: ICU4XLineBreakIteratorUtf16 segment_utf16(const diplomat::span<uint16_t> input) const

        Segments a UTF-16 string.

        See the `Rust documentation for segment_utf16 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.segment_utf16>`__ for more information.

        Lifetimes: ``this``, ``input`` must live at least as long as the output.


    .. cpp:function:: ICU4XLineBreakIteratorLatin1 segment_latin1(const diplomat::span<uint8_t> input) const

        Segments a Latin-1 string.

        See the `Rust documentation for segment_latin1 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.segment_latin1>`__ for more information.

        Lifetimes: ``this``, ``input`` must live at least as long as the output.


.. cpp:enum-struct:: ICU4XWordBreakRule

    See the `Rust documentation for WordBreakRule <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/enum.WordBreakRule.html>`__ for more information.


    .. cpp:enumerator:: Normal

    .. cpp:enumerator:: BreakAll

    .. cpp:enumerator:: KeepAll
