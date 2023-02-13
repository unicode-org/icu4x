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


.. cpp:class:: ICU4XWordSegmenter

    An ICU4X word-break segmenter, capable of finding word breakpoints in strings.

    See the `Rust documentation for WordSegmenter <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordSegmenter.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XWordSegmenter, ICU4XError> create_auto(const ICU4XDataProvider& provider)

        Construct an :cpp:class:`ICU4XWordSegmenter` with automatically selecting the best available LSTM or dictionary payload data.

        Note: currently, it uses dictionary for Chinese and Japanese, and LSTM for Burmese, Khmer, Lao, and Thai.

        See the `Rust documentation for try_new_auto_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordSegmenter.html#method.try_new_auto_unstable>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XWordSegmenter, ICU4XError> create_lstm(const ICU4XDataProvider& provider)

        Construct an :cpp:class:`ICU4XWordSegmenter` with LSTM payload data for Burmese, Khmer, Lao, and Thai.

        Warning: :cpp:class:`ICU4XWordSegmenter` created by this function doesn't handle Chinese or Japanese.

        See the `Rust documentation for try_new_lstm_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordSegmenter.html#method.try_new_lstm_unstable>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XWordSegmenter, ICU4XError> create_dictionary(const ICU4XDataProvider& provider)

        Construct an :cpp:class:`ICU4XWordSegmenter` with dictionary payload data for Chinese, Japanese, Burmese, Khmer, Lao, and Thai.

        See the `Rust documentation for try_new_dictionary_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordSegmenter.html#method.try_new_dictionary_unstable>`__ for more information.


    .. cpp:function:: ICU4XWordBreakIteratorUtf8 segment_utf8(const std::string_view input) const

        Segments a (potentially ill-formed) UTF-8 string.

        See the `Rust documentation for segment_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordSegmenter.html#method.segment_utf8>`__ for more information.

        Lifetimes: ``this``, ``input`` must live at least as long as the output.


    .. cpp:function:: ICU4XWordBreakIteratorUtf16 segment_utf16(const diplomat::span<const uint16_t> input) const

        Segments a UTF-16 string.

        See the `Rust documentation for segment_utf16 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordSegmenter.html#method.segment_utf16>`__ for more information.

        Lifetimes: ``this``, ``input`` must live at least as long as the output.


    .. cpp:function:: ICU4XWordBreakIteratorLatin1 segment_latin1(const diplomat::span<const uint8_t> input) const

        Segments a Latin-1 string.

        See the `Rust documentation for segment_latin1 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordSegmenter.html#method.segment_latin1>`__ for more information.

        Lifetimes: ``this``, ``input`` must live at least as long as the output.

