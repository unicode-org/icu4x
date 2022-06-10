``errors::ffi``
===============

.. cpp:enum-struct:: ICU4XError

    A common enum for errors that ICU4X may return, organized by API
    The error names are stable and can be checked against as strings in the JS API

    .. cpp:enumerator:: WriteableError

        An error arising from writing to a string Typically found when not enough space is allocated

    .. cpp:enumerator:: LocaleUndefinedSubtagError

        The subtag being requested was not set

    .. cpp:enumerator:: LocaleParserError

        The locale or subtag string failed to parse
