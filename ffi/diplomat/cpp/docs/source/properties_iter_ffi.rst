``properties_iter::ffi``
========================

.. cpp:class:: CodePointRangeIterator

    An iterator over code point ranges, produced by ``ICU4XCodePointSetData`` or one of the ``ICU4XCodePointMapData`` types


    .. cpp:function:: CodePointRangeIteratorResult next()


.. cpp:struct:: CodePointRangeIteratorResult

    Result of a single iteration of :cpp:class:`CodePointRangeIterator`. Logically can be considered to be an ``Option<RangeInclusive<u32>>``,

    ``start`` and ``end`` represent an inclusive range of code points start, end, and ``done`` will be true when the iterator finishes.


    .. cpp:member:: uint32_t start

    .. cpp:member:: uint32_t end

    .. cpp:member:: bool done
