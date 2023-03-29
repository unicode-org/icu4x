``properties_iter::ffi``
========================

.. js:class:: CodePointRangeIterator

    An iterator over code point ranges, produced by ``ICU4XCodePointSetData`` or one of the ``ICU4XCodePointMapData`` types


    .. js:method:: next()

.. js:class:: CodePointRangeIteratorResult

    Result of a single iteration of :js:class:`CodePointRangeIterator`. Logically can be considered to be an ``Option<RangeInclusive<u32>>``,

    ``start`` and ``end`` represent an inclusive range of code points start, end, and ``done`` will be true when the iterator finishes.


    .. js:attribute:: start

    .. js:attribute:: end

    .. js:attribute:: done
