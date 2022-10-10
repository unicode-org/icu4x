``segmenter_sentence::ffi``
===========================

.. js:class:: ICU4XSentenceBreakIteratorLatin1

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. js:class:: ICU4XSentenceBreakIteratorUtf16

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. js:class:: ICU4XSentenceBreakIteratorUtf8

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. js:class:: ICU4XSentenceSegmenter

    An ICU4X sentence-break segmenter, capable of finding sentence breakpoints in strings.

    See the `Rust documentation for SentenceSegmenter <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceSegmenter.html>`__ for more information.


    .. js:function:: create(provider)

        Construct an :js:class:`ICU4XSentenceSegmenter`.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceSegmenter.html#method.try_new_unstable>`__ for more information.


    .. js:function:: segment_utf8(input)

        Segments a (potentially ill-formed) UTF-8 string.

        See the `Rust documentation for segment_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceSegmenter.html#method.segment_utf8>`__ for more information.


    .. js:function:: segment_utf16(input)

        Segments a UTF-16 string.

        See the `Rust documentation for segment_utf16 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceSegmenter.html#method.segment_utf16>`__ for more information.

        - Note: ``input`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.


    .. js:function:: segment_latin1(input)

        Segments a Latin-1 string.

        See the `Rust documentation for segment_latin1 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceSegmenter.html#method.segment_latin1>`__ for more information.

        - Note: ``input`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.

