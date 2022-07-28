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


.. js:class:: ICU4XSentenceBreakSegmenter

    An ICU4X sentence-break segmenter, capable of finding sentence breakpoints in strings.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html>`__ for more information.


    .. js:staticfunction:: try_new(provider)

        Construct an :js:class:`ICU4XSentenceBreakSegmenter`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.try_new>`__ for more information.


    .. js:function:: segment_utf8(input)

        Segments a UTF-8 string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.segment_str>`__ for more information.


    .. js:function:: segment_utf16(input)

        Segments a UTF-16 string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.segment_utf16>`__ for more information.


        - Note: ``input`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.

    .. js:function:: segment_latin1(input)

        Segments a Latin-1 string.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.SentenceBreakSegmenter.html#method.segment_latin1>`__ for more information.


        - Note: ``input`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.
