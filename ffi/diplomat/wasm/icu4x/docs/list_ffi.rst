``list::ffi``
=============

.. js:class:: ICU4XList

    A list of strings


    .. js:staticfunction:: new()

        Create a new list of strings


    .. js:staticfunction:: with_capacity(capacity)

        Create a new list of strings with preallocated space to hold at least ``capacity`` elements


    .. js:function:: push(val)

        Push a string to the list

        For C++ users, potentially invalid UTF8 will be handled via REPLACEMENT CHARACTERs


    .. js:function:: len()

        The number of elements in this list


.. js:class:: ICU4XListFormatter

    See the `Rust documentation for ListFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/list/struct.ListFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new_and(provider, locale, style)

        Construct a new ICU4XListFormatter instance for And patterns

        See the `Rust documentation for try_new_and_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ListFormatter.html#method.try_new_and_unstable>`__ for more information.


    .. js:staticfunction:: try_new_or(provider, locale, style)

        Construct a new ICU4XListFormatter instance for And patterns

        See the `Rust documentation for try_new_or_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ListFormatter.html#method.try_new_or_unstable>`__ for more information.


    .. js:staticfunction:: try_new_unit(provider, locale, style)

        Construct a new ICU4XListFormatter instance for And patterns

        See the `Rust documentation for try_new_unit_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ListFormatter.html#method.try_new_unit_unstable>`__ for more information.


    .. js:function:: format(list)

        See the `Rust documentation for format <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ListFormatter.html#method.format>`__ for more information.


.. js:class:: ICU4XListStyle

    See the `Rust documentation for ListStyle <https://unicode-org.github.io/icu4x-docs/doc/icu/list/enum.ListStyle.html>`__ for more information.

