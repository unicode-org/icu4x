``segmenter_grapheme::ffi``
===========================

.. js:class:: ICU4XGraphemeClusterBreakIteratorLatin1

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. js:class:: ICU4XGraphemeClusterBreakIteratorUtf16

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. js:class:: ICU4XGraphemeClusterBreakIteratorUtf8

    .. js:function:: next()

        Finds the next breakpoint. Returns -1 if at the end of the string or if the index is out of range of a 32-bit signed integer.


.. js:class:: ICU4XGraphemeClusterBreakSegmenter

    An ICU4X grapheme-cluster-break segmenter, capable of finding grapheme cluster breakpoints in strings.

    See the `Rust documentation for GraphemeClusterBreakSegmenter <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html>`__ for more information.


    .. js:staticfunction:: try_new(provider)

        Construct an :js:class:`ICU4XGraphemeClusterBreakSegmenter`.

        See the `Rust documentation for try_new <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.try_new>`__ for more information.


    .. js:function:: segment_utf8(input)

        Segments a UTF-8 string.

        See the `Rust documentation for segment_str <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_str>`__ for more information.


    .. js:function:: segment_utf16(input)

        Segments a UTF-16 string.

        See the `Rust documentation for segment_utf16 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_utf16>`__ for more information.


        - Note: ``input`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.

    .. js:function:: segment_latin1(input)

        Segments a Latin-1 string.

        See the `Rust documentation for segment_latin1 <https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_latin1>`__ for more information.


        - Note: ``input`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.
