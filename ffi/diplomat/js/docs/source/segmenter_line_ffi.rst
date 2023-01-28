``segmenter_line::ffi``
=======================

.. js:class:: ICU4XLineBreakIteratorLatin1

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. js:class:: ICU4XLineBreakIteratorUtf16

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. js:class:: ICU4XLineBreakIteratorUtf8

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. js:class:: ICU4XLineBreakOptionsV1

    See the `Rust documentation for LineBreakOptions <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineBreakOptions.html>`__ for more information.


    .. js:attribute:: line_break_rule

    .. js:attribute:: word_break_rule

    .. js:attribute:: ja_zh

.. js:class:: ICU4XLineBreakRule

    See the `Rust documentation for LineBreakRule <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/enum.LineBreakRule.html>`__ for more information.


.. js:class:: ICU4XLineSegmenter

    An ICU4X line-break segmenter, capable of finding breakpoints in strings.

    See the `Rust documentation for LineSegmenter <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html>`__ for more information.


    .. js:function:: create_auto(provider)

        Construct a :js:class:`ICU4XLineSegmenter` with default options. It automatically loads the best available payload data for Burmese, Khmer, Lao, and Thai.

        See the `Rust documentation for try_new_auto_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.try_new_auto_unstable>`__ for more information.


    .. js:function:: create_lstm(provider)

        Construct a :js:class:`ICU4XLineSegmenter` with default options and LSTM payload data for Burmese, Khmer, Lao, and Thai.

        See the `Rust documentation for try_new_lstm_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.try_new_lstm_unstable>`__ for more information.


    .. js:function:: create_dictionary(provider)

        Construct a :js:class:`ICU4XLineSegmenter` with default options and dictionary payload data for Burmese, Khmer, Lao, and Thai..

        See the `Rust documentation for try_new_dictionary_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.try_new_dictionary_unstable>`__ for more information.


    .. js:function:: create_auto_with_options_v1(provider, options)

        Construct a :js:class:`ICU4XLineSegmenter` with custom options. It automatically loads the best available payload data for Burmese, Khmer, Lao, and Thai.

        See the `Rust documentation for try_new_auto_with_options_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.try_new_auto_with_options_unstable>`__ for more information.


    .. js:function:: create_lstm_with_options_v1(provider, options)

        Construct a :js:class:`ICU4XLineSegmenter` with custom options and LSTM payload data for Burmese, Khmer, Lao, and Thai.

        See the `Rust documentation for try_new_lstm_with_options_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.try_new_lstm_with_options_unstable>`__ for more information.


    .. js:function:: create_dictionary_with_options_v1(provider, options)

        Construct a :js:class:`ICU4XLineSegmenter` with custom options and dictionary payload data for Burmese, Khmer, Lao, and Thai.

        See the `Rust documentation for try_new_dictionary_with_options_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.try_new_dictionary_with_options_unstable>`__ for more information.


    .. js:function:: segment_utf8(input)

        Segments a (potentially ill-formed) UTF-8 string.

        See the `Rust documentation for segment_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.segment_utf8>`__ for more information.


    .. js:function:: segment_utf16(input)

        Segments a UTF-16 string.

        See the `Rust documentation for segment_utf16 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.segment_utf16>`__ for more information.

        - Note: ``input`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.


    .. js:function:: segment_latin1(input)

        Segments a Latin-1 string.

        See the `Rust documentation for segment_latin1 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.LineSegmenter.html#method.segment_latin1>`__ for more information.

        - Note: ``input`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.


.. js:class:: ICU4XWordBreakRule

    See the `Rust documentation for WordBreakRule <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/enum.WordBreakRule.html>`__ for more information.

